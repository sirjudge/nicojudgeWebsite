use crate::{
    components::{ResourceNotFound, UnexpectedError},
    models::BlogPost,
    Route,
};
use dioxus::{
    logger::tracing::{debug, error, field::debug, info, warn},
    prelude::*,
};

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    debug!("Rendering blog post with id: {id}");
    match BlogPost::get_post_by_id(id) {
        // If the post is found, we can render it as HTML
        Ok(Some(post)) => {
            debug!("Blog post with id {id} found: {post:?}");
            return rsx! {
                // div { id: "blog-post", dangerously_set_inner_html: post.to_html() }
                div {
                    class: "blog-post",
                    id: "blog-post-{id}",
                    h1 {
                        class: "blog-post-title",
                        id: "blog-post-title-{id}",
                        "{post.title}"
                    }
                    //TODO: this isn't rendering the HTML correctly. I think
                    //it's just dipslaying the HTML as raw text instead of rednering
                    //the actual html content
                    p {
                        class: "blog-post-content",
                        "{post.content}"
                    }
                }
            };
        }
        // If the post is not found, render a "not found" element
        Ok(None) => {
            warn!("Blog post with id {id} not found");
            return rsx! {
                ResourceNotFound {}
            };
        }
        // If there was an error getting the post, we can log it and render a generic server error
        // message
        Err(e) => {
            error!("Error getting blog post with id {id}: {e}");
            return rsx! {
                UnexpectedError {}
            };
        }
    }
}
