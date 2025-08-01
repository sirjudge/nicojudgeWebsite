use crate::components::BlogPostFormData;
use dioxus::{
    logger::tracing::{error, info},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

/// This is a separation of the model used for the database and the model used for the API.
/// This allows us to eliminate the SQLx dependency from the frontend layer and ensures
/// that database-specific code only exists in server builds.
pub struct BlogPostModel {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct BlogPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

impl BlogPost {
    /// Converts a BlogPost to a BlogPostModel
    ///
    /// This method provides a clean conversion from the database entity
    /// to the frontend model, ensuring separation of concerns.
    pub fn to_model(&self) -> BlogPostModel {
        BlogPostModel {
            id: self.id,
            title: self.title.clone(),
            content: self.content.clone(),
        }
    }

    /// Creates a BlogPost from form data by converting form
    /// input data into a BlogPost struct ready for database insertion.
    ///
    /// # Arguments
    /// * `form_data` - The form data from the frontend
    pub fn from_form_data(form_data: BlogPostFormData) -> BlogPost {
        BlogPost {
            id: None,
            title: form_data.title,
            content: form_data.content,
        }
    }
}

#[server]
pub async fn get_post_list() -> Result<Vec<BlogPost>, ServerFnError> {
    let mut return_list: Vec<BlogPost> = Vec::new();
    match create_connection().await {
        Ok(mut conn) => {
            let mut stream =
                sqlx::query_as::<_, BlogPost>("SELECT id, title, content FROM blog_posts")
                    .fetch_all(&mut conn)
                    .await;

            stream
                .into_iter()
                .for_each(|mut post| return_list.append(&mut post));

            Ok(return_list)
        }
        Err(error) => Err(ServerFnError::new(format!(
            "uh oh error occurred extrating list of posts:{:?}",
            error
        ))),
    }
}

/// Retrieves a blog post by ID by fetching a blog post from the database using its ID.
/// It uses SQLx for async database operations and provides proper error handling.
///
/// # Arguments
/// * `post_id` - The ID of the blog post to retrieve
///
/// # Returns
/// A `Result` containing either `Some(BlogPost)` if found, `None` if not found,
/// or a `ServerFnError` if an error occurs.
///
/// # Examples
/// ```rust
/// let post = get_post_by_id(1).await?;
/// ```
#[server]
pub async fn get_post_by_id(post_id: i32) -> Result<Option<BlogPost>, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query_as::<_, BlogPost>(
                "SELECT id, title, content FROM blog_posts WHERE id = ?1",
            )
            .bind(post_id)
            .fetch_optional(&mut conn)
            .await;

            match result {
                Ok(post) => {
                    if post.is_some() {
                        info!("Post found with id: {post_id}");
                    } else {
                        info!("No post found with id: {post_id}");
                    }
                    Ok(post)
                }
                Err(e) => {
                    error!("Error loading blog post: {e}");
                    Err(ServerFnError::new(format!("Error loading blog post: {e}")))
                }
            }
        }
        Err(e) => Err(ServerFnError::new(format!(
            "Database connection error: {e}"
        ))),
    }
}

/// Saves a new blog post to the database by inserting a new blog post into
/// the database and returns the created post with its assigned ID.
///
/// # Arguments
/// * `blog_post_to_save` - The blog post data to save
///
/// # Returns
/// A `Result` containing either `Some(BlogPost)` with the saved post data,
/// or a `ServerFnError` if an error occurs.
///
/// # Examples
///
/// ```rust
/// let new_post = BlogPost::from_form_data(form_data);
/// let saved_post = save_post(new_post).await?;
/// ```
#[server]
pub async fn save_post(blog_post_to_save: BlogPost) -> Result<Option<BlogPost>, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                "INSERT INTO blog_posts (title, content) VALUES (?1, ?2)",
                blog_post_to_save.title,
                blog_post_to_save.content
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(query_result) => {
                    let inserted_id = query_result.last_insert_rowid() as i32;
                    info!("Blog post saved with id: {inserted_id}");
                    Ok(Some(BlogPost {
                        id: Some(inserted_id),
                        title: blog_post_to_save.title,
                        content: blog_post_to_save.content,
                    }))
                }
                Err(e) => {
                    Err(ServerFnError::new(format!(
                        "Error occurred during blog insert: {e}"
                    )))
                }
            }
        }
        Err(e) => {
            Err(ServerFnError::new(format!(
                "Database connection error: {e}"
            )))
        }
    }
}
