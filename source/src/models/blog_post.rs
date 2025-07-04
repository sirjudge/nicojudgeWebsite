use diesel::{SqliteConnection,prelude::*};
use dioxus::{
    prelude::{server, ServerFnError},
    logger::tracing::{error, info}
};
use crate::schema::blog_posts;
use crate::schema::blog_posts::dsl::*;


#[derive(Queryable, Insertable,Selectable,Debug, Clone)]
#[diesel(table_name = blog_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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

    fn establish_connection() -> Result<SqliteConnection, ServerFnError> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .map_err(|_| ServerFnError::new("Failed to connect to the database"))
    }

    pub fn get_post_by_id(post_id: i32) -> Result<Option<Self>, ServerFnError> {
        let mut connection = Self::establish_connection()?;
        info!("Connected to db successfully, extracting post with id: {post_id}");

        let posts = blog_posts
            .filter(id.eq(post_id))
            .limit(1)
            .select(BlogPost::as_select())
            .load(&mut connection)
            .map_err(|e| ServerFnError::new("Error loading blog post"))?;

        if posts.is_empty() {
            info!("No post found with id: {post_id}");
            return Ok(None);
        }

        info!("Post found with id: {post_id}");
        Ok(posts.into_iter().next())
    }

    pub fn to_html(&self) -> String {
        format!(
            "<h1>{}</h1><p>{}</p>",
            self.title, self.content
        )
    }
}
