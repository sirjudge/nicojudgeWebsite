use crate::{
    models::{get_post_list, BlogPost},
    route::Route,
};
use dioxus::{
    logger::tracing::{debug, error, info, warn},
    prelude::*,
};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

#[component]
pub fn blog_table_rows(blog_list: Vec<BlogPost>) -> Element {
    if blog_list.is_empty() {
        return rsx! {
            p { "No posts available" }
        };
    }

    rsx! {
        for post in blog_list.iter() {
            tr {
                class: "blog_row",
                td { "{post.id.unwrap_or(-1)}" }
                td {
                    Link {
                        to: Route::Blog { id:post.id.unwrap_or(-1) },
                        "{post.title}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogTableOfContents() -> Element {
    let blog_post_list = use_resource(move || async move { get_post_list().await });

    // this is an unholy abomination
    // this is done to get this thing to compile first
    // but I need to come back and fix this
    match &*blog_post_list.clone().read() {
        Some(post_list) => match post_list {
            Ok(post_list) => {
                return rsx! {
                    table {
                        tr {
                            th { "Id" }
                            th { "Title" }
                        }
                        blog_table_rows { blog_list: post_list.clone() }
                    }
                }
            }
            Err(error) => {
                error!("Error ocurred extracting blog list: {:?}", error);
                return rsx! { p { "ah sheesh error on the backend sorry!" } };
            }
        },
        None => {
            return rsx! {
                p { " no blog posts found" }
            };
        }
    }
}
