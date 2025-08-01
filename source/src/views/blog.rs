use std::vec;

use crate::{
    components::{ResourceNotFound},
    models::{get_post_by_id, BlogPostModel},
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
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    let post_resource = use_resource(move || async move { get_blog_model(id).await });

    let x = match &*post_resource.read() {
        Some(Some(post)) => {
            rsx! {
                // div { id: "blog-post", dangerously_set_inner_html: post.to_html() }
                div {
                    class: "blog-post",
                    id: "blog-post-{id}",
                    h1 {
                        class: "blog-post-title",
                        id: "blog-post-title-{id}",
                        "{post.title}"
                    }
                    //TODO: Maybe don't use danger_inner_html but eh what's the worst that could
                    //happen
                    p {
                        class: "blog-post-content",
                        dangerous_inner_html:post.content.to_string()
                    }
                }
            }
        }
        Some(None) => {
            warn!("Blog post with id {id} not found");
            rsx! {
                ResourceNotFound {}
            }
        }
        None => {
            rsx! {
                p { "Loading blog post..." }
            }
        }
    };
    x
}
