use std::vec;

use crate::{
    components::{ResourceNotFound, UnexpectedError},
    models::{get_post_by_id, BlogPost, BlogPostModel},
};

use dioxus::{
    logger::tracing::{debug, error, field::debug, warn},
    prelude::*,
};

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

pub async fn get_blog_model(post_id: i32) -> Option<BlogPostModel> {
    match get_post_by_id(post_id).await {
        Ok(Some(post)) => {
            debug!("Blog post with id {post_id} found: {post:?}");
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
#[server]
pub async fn get_blog_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    let blog_post_1 = BlogPost {
        id : Some(1),
        title: "Cool title 1",
        content: "This is content 1"
    };
    let blog_post_2 = BlogPost {
        id : Some(2),
        title: "Cool title 2",
        content: "This is content 2"
    };

    Ok(vec![blog_post_1,blog_post_2])
}

#[component]
pub fn blog_table_rows(blog_list: Vec<BlogPost>) -> Element {
    if blog_list.is_empty() {
        return rsx! {
            p { "No posts available" }
        }
    }

    return rsx! {
        for post in blog_list.iter() {
            tr {
                class: "blog_row",
                td { "{post.title}" }
                //BUG: need to handle if Id is none but rust analyzer gets confused here
                // td { "{post.id.}" }
                td { "totally 1" }
            }
        }
    };
}


#[component]
pub fn BlogTableOfContents() -> Element {
    debug!("Extracting blog table data now");
    let blog_post_list = use_resource(move || async move { get_blog_posts().await });
    let blog_post_list= blog_post_list.read();
    // this is of type
    // GenerationalRef<Ref<'_, Option<Result<Vec<BlogPost>, ServerFnError>>>>
    // but I want it to just be type Result<Vec<BlogPost>, ServerFnError>>
    // or maybe just Vec<BlogPost>
    if blog_post_list.is_some() {
        return rsx! {
            table {
                thead {
                    tr{
                        th { "title" }
                        th { "name" }
                    }
                }
                tbody {
                    tr {
                        class: "post-row",
                        td { "supposed title".to_string() }
                        td { "supposed content".to_string() }
                    }
                    // blog_table_rows()
                }
            }
        };
    }
    else {
        rsx! {
            p { "no blog posts currently available" }
        }
    }
}

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    debug!("Rendering blog post with id: {id}");
    // Use use_resource to call the server function
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
                    //TODO: this isn't rendering the HTML correctly. I think
                    //it's just dipslaying the HTML as raw text instead of rednering
                    //the actual html content
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
            debug!("Loading blog post with id {id}");
            rsx! {
                div {
                    class: "loading",
                    p { "Loading blog post..." }
                }
            }
        }
    };
    x
}
