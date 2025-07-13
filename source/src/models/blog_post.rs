#[cfg(feature = "server")]
use crate::schema::blog_posts;
#[cfg(feature = "server")]
use crate::schema::blog_posts::dsl::*;
#[cfg(feature = "server")]
use diesel::{prelude::*, SqliteConnection};
use dioxus::{
    logger::tracing::{error, info},
    // prelude::{server, ServerFnError},
    prelude::*,
};
use serde::{Deserialize, Serialize};

/// This is a separation of the model used for the database and the model used for the API.
/// so we can eliminate the diesel dependency from the frontend layer
/// Not even going to argue why this is a good idea because it's not
/// I'm just trying to compile it when I build for the web because
/// there shouldn't be any diesel code on the front
pub struct BlogPostModel {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = blog_posts))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct BlogPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

impl BlogPost {
    pub fn to_model(&self) -> BlogPostModel {
        BlogPostModel {
            id: self.id.unwrap_or(0),
            title: self.title.clone(),
            content: self.content.clone(),
        }
    }
}

#[server]
pub async fn get_post_by_id(post_id: i32) -> Result<Option<BlogPost>, ServerFnError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match SqliteConnection::establish(&database_url)
        .map_err(|_| ServerFnError::new("Failed to connect to the database"))
    {
        Ok(mut connection) => {
            info!("Successfully connected to the database");
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
        Err(e) => {
            return Err(ServerFnError::new(format!(
                "Database connection error: {e}"
            )));
        }
    }
}
