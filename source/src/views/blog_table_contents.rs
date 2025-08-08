use crate::{
    models::{get_post_list, BlogPost},
    route::Route,
    views::Navbar,
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
pub fn BlogTableOfContents() -> Element {
    let blog_post_list = use_resource(move || async move { get_post_list().await });

    rsx! {
        div {
            class: "blog-table-contents",
            h2 { 
                style: "color: #ffffff; margin-bottom: 20px;",
                "Blog Posts" 
            }
            
            match &*blog_post_list.read() {
                Some(Ok(post_list)) => {
                    if post_list.is_empty() {
                        rsx! {
                            div {
                                style: "text-align: center; padding: 40px; color: #888; background: #1a1a1a; border-radius: 8px;",
                                p { "No blog posts available yet." }
                            }
                        }
                    } else {
                        rsx! {
                            table {
                                style: "
                                    width: 100%; 
                                    border-collapse: collapse; 
                                    margin-top: 20px;
                                    background-color: #1a1a1a;
                                    color: #ffffff;
                                    border-radius: 8px;
                                    overflow: hidden;
                                ",
                                thead {
                                    style: "background-color: #2a2a2a;",
                                    tr {
                                        th { 
                                            style: "
                                                padding: 12px;
                                                text-align: left;
                                                border-bottom: 2px solid #444;
                                                font-weight: bold;
                                                width: 80px;
                                            ",
                                            "ID" 
                                        }
                                        th { 
                                            style: "
                                                padding: 12px;
                                                text-align: left;
                                                border-bottom: 2px solid #444;
                                                font-weight: bold;
                                            ",
                                            "Title" 
                                        }
                                    }
                                }
                                tbody {
                                    style: "background-color: #1a1a1a;",
                                    for post in post_list.iter() {
                                        tr {
                                            class: "blog-row",
                                            style: "
                                                border-bottom: 1px solid #333;
                                                transition: background-color 0.2s ease;
                                            ",
                                            onmouseenter: move |_| {},
                                            onmouseleave: move |_| {},
                                            td { 
                                                style: "
                                                    padding: 12px; 
                                                    border-bottom: 1px solid #333;
                                                    color: #888;
                                                    font-family: monospace;
                                                ",
                                                "{post.id.unwrap_or(-1)}" 
                                            }
                                            td {
                                                style: "padding: 12px; border-bottom: 1px solid #333;",
                                                Link {
                                                    to: Route::Blog { id: post.id.unwrap_or(-1) },
                                                    style: "
                                                        color: #4a9eff; 
                                                        text-decoration: none;
                                                        font-weight: 500;
                                                    ",
                                                    "{post.title}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(error)) => {
                    error!("Error occurred extracting blog list: {:?}", error);
                    rsx! {
                        div {
                            style: "
                                color: #ff6b6b; 
                                padding: 20px; 
                                background: #2a1a1a; 
                                border-radius: 8px; 
                                border-left: 4px solid #ff6b6b;
                                margin-top: 20px;
                            ",
                            p { "Error loading blog posts. Please try again later." }
                        }
                    }
                },
                None => rsx! {
                    div {
                        style: "text-align: center; padding: 40px; color: #888;",
                        p { "Loading blog posts..." }
                    }
                }
            }
        }
    }
}
