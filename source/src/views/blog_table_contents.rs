use crate::models::BlogPost;
use dioxus::prelude::*;
use dioxus::logger::tracing::{debug, info, warn, error};

#[server]
pub async fn get_blog_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    let blog_post_1 = BlogPost {
        id : Some(1),
        title: "Cool title 1".to_string(),
        content: "This is content 1".to_string()
    };
    let blog_post_2 = BlogPost {
        id : Some(2),
        title: "Cool title 2".to_string(),
        content: "This is content 2".to_string()
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

    rsx! {
        for post in blog_list.iter() {
            tr {
                class: "blog_row",
                td { "{post.title}" }
                //BUG: need to handle if Id is none but rust analyzer gets confused here
                // td { "{post.id.}" }
                td { "id: totally 1" }
            }
        }
    }
}


#[component]
pub fn BlogTableOfContents() -> Element {
    debug!("Extracting blog table data now");
    let blog_post_list = use_resource(move || async move { get_blog_posts().await });

    // this is an unholy abomination
    // this is done to get this thing to compile first
    // but I need to come back and fix this
    match &*blog_post_list.clone().read() {
        Some(post_list) => {
            match post_list {
                Ok(post_list) => {
                    return rsx! {
                        blog_table_rows { blog_list: post_list.clone() }
                    }
                }
                Err(error) => {
                    error!("Error ocurred extracting blog list: {:?}", error);
                    return rsx! { p { "ah sheesh error on the backend sorry!" } };
                }
            }
        }
        None => {
            return rsx! {
                p { " no blog posts found" }
            };
        }
    }
}
