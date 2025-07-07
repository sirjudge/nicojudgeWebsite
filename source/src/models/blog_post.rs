#[cfg(feature = "server")]
use diesel::{SqliteConnection, prelude::*};
use dioxus::{
    prelude::{server, ServerFnError},
    logger::tracing::{error, info}
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::schema::blog_posts;
#[cfg(feature = "server")]
use crate::schema::blog_posts::dsl::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = blog_posts))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct BlogPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

impl BlogPost {
    pub fn new(title_str: String, content_str: String) -> Self {
        BlogPost {
            id: None,
            title: title_str,
            content: content_str,
        }
    }

    #[cfg(feature = "server")]
    fn establish_connection() -> Result<SqliteConnection, ServerFnError> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .map_err(|_| ServerFnError::new("Failed to connect to the database"))
    }

    #[cfg(feature = "server")]
    fn get_post_by_id_impl(post_id: i32) -> Result<Option<Self>, ServerFnError> {
        let mut connection = Self::establish_connection()?;
        info!("Connected to db successfully, extracting post with id: {post_id}");

        let posts = blog_posts
            .filter(id.eq(post_id))
            .limit(1)
            .select(BlogPost::as_select())
            .load(&mut connection)
            .map_err(|e| ServerFnError::new(format!("Error loading blog post: {e}")))?;

        if posts.is_empty() {
            info!("No post found with id: {post_id}");
            return Ok(None);
        }

        info!("Post found with id: {post_id}");
        Ok(posts.into_iter().next())
    }
}

/// Server function to get a blog post by ID
/// This function runs on the server and can be called from the client
#[server]
pub async fn get_blog_post_by_id(post_id: i32) -> Result<Option<BlogPost>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        BlogPost::get_post_by_id_impl(post_id)
    }
    #[cfg(not(feature = "server"))]
    {
        // This should never be called on the client side, but just in case
        Err(ServerFnError::new("Database operations not supported on client side"))
    }
}
