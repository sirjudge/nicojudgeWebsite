use std::vec;

use crate::{
    components::{ResourceNotFound},
    models::{get_post_by_id, BlogPostModel},
    views::Navbar,
};

use dioxus::{
    logger::tracing::{error, warn},
    prelude::*,
};

//TODO: Should revisit this later
// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

pub async fn get_blog_model(post_id: i32) -> Option<BlogPostModel> {
    match get_post_by_id(post_id).await {
        Ok(Some(post)) => {
            Some(post.to_model())
        }
        Ok(None) => {
            warn!("Blog post with id {post_id} not found");
            None
        }
        Err(e) => {
            error!("Error fetching blog post with id {post_id}: {e}");
            None
        }
    }
}

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// Now accepts an ID parameter to load the specific blog post
#[component]
pub fn Blog(id: i32) -> Element {
    let blog_post = use_resource(move || async move { get_blog_model(id).await });

    rsx! {
        Navbar {}
        div {
            class: "blog-post",
            style: "
                max-width: 800px;
                margin: 0 auto;
                padding: 20px;
                background: #1a1a1a;
                border-radius: 8px;
                margin-top: 20px;
            ",
            
            match &*blog_post.read() {
                Some(Some(post)) => rsx! {
                    article {
                        style: "color: #ffffff;",
                        header {
                            style: "margin-bottom: 30px; border-bottom: 2px solid #333; padding-bottom: 20px;",
                            h1 {
                                style: "
                                    color: #4a9eff;
                                    margin: 0 0 10px 0;
                                    font-size: 2.5rem;
                                    line-height: 1.2;
                                ",
                                "{post.title}"
                            }
                            div {
                                style: "color: #888; font-size: 14px;",
                                "Post ID: {id}"
                            }
                        }
                        div {
                            style: "
                                line-height: 1.6;
                                font-size: 1.1rem;
                                color: #e0e0e0;
                            ",
                            dangerous_inner_html: "{post.content}"
                        }
                    }
                },
                Some(None) => rsx! {
                    div {
                        style: "
                            text-align: center; 
                            padding: 40px; 
                            color: #ff6b6b;
                            background: #2a1a1a;
                            border-radius: 8px;
                            border-left: 4px solid #ff6b6b;
                        ",
                        h2 {
                            style: "color: #ff6b6b; margin-bottom: 15px;",
                            "Blog Post Not Found"
                        }
                        p {
                            style: "color: #888; margin-bottom: 10px;",
                            "The blog post with ID {id} could not be found."
                        }
                        p {
                            style: "color: #666; font-size: 14px;",
                            "It may have been moved or deleted."
                        }
                    }
                },
                None => rsx! {
                    div {
                        style: "text-align: center; padding: 40px; color: #888;",
                        p { "Loading blog post..." }
                    }
                }
            }
        }
    }
}
