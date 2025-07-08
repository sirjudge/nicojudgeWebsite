use crate::{
    components::{ResourceNotFound, UnexpectedError},
    models::get_post_by_id,
};

use dioxus::{
    logger::tracing::{debug, error, warn},
    prelude::*,
};

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    debug!("Rendering blog post with id: {id}");

    // Use use_resource to call the server function
    let post_resource = use_resource(move || async move { get_post_by_id(id).await });
    let post_data = post_resource.read();
    match post_data.as_ref() {
        Some(Ok(Some(post))) => {
            debug!("Blog post with id {id} found: {post:?}");
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
                    //TODO: this isn't rendering the HTML correctly. I think
                    //it's just dipslaying the HTML as raw text instead of rednering
                    //the actual html content
                    p {
                        class: "blog-post-content",
                        "{post.content}"
                    }
                }
            }
        }
        Some(Ok(None)) => {
            warn!("Blog post with id {id} not found");
            rsx! {
                ResourceNotFound {}
            }
        }
        Some(Err(e)) => {
            error!("Error getting blog post with id {id}: {e}");
            rsx! {
                UnexpectedError {}
            }
        }
        None => {
            debug!("Loading blog post with id {id}");
            rsx! {
                div {
                    class: "loading",
                    p { "Loading blog post..." }
                }
            }
        }
    }
}
