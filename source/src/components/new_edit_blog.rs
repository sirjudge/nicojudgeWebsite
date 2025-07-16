use crate::models::{save_post, BlogPost, BlogPostModel};

use dioxus::{
    html::textarea, logger::tracing::{debug, error, info, warn}, prelude::{server_fn::{error::Result, ServerFn}, *}
};
use serde::{Deserialize, Serialize};
use std::{default, fmt::{Display}, fmt};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlogPostFormData {
    pub title: String,
    pub content: String,
}

impl BlogPostFormData {
    fn new() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
        }
    }

    fn to_string(&self) -> String {
        format!("title: {} post:{}", self.title, self.content)
    }
}

// #[server]
// pub async fn save_blog_post() -> Result<BlogPost, ServerFnError> {
//     let model = BlogPost {
//         id: None,
//         title: "Test title".to_string(),
//         content: "test content".to_string(),
//     };
//
//     Ok(model)
// }

#[server]
pub async fn save_new_post(blog_form_data: BlogPostFormData) -> Result<Option<BlogPost>, ServerFnError> {
    let blog_post = BlogPost::from_form_data(blog_form_data);
    save_post(blog_post).await
    // save_post(blog_post)
}

//TODO: Can find an example of form validation in dioxus here:
//https://github.com/DioxusLabs/dioxus/blob/main/examples/form.rs
#[component]
pub fn NewEditBlog() -> Element {
    let mut post_title = use_signal(|| "".to_string());
    let mut post_content = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "new-edit-blog",
            h1 { "New/Edit Blog Post" }
            form {
                id: "newEditBlogForm",
                style: "display:flex; flex-direction:column;",
                onsubmit:  move |input_event| {
                    let title = post_title.read();
                    let content = post_content.read();
                    info!("title: {:#?} content: {:#?}", title, content);
                },
                label {
                    "Post Title:"
                },
                input {
                    r#type: "text",
                    placeholder: "Title",
                    name: "title",
                    required: true,
                    oninput: move |input_event| {
                        post_title.set(input_event.value().clone());
                    }

                },
                label {
                    "Post Content:"
                },
                textarea {
                    placeholder: "Content",
                    name: "content",
                    required: true,
                    oninput: move |input_event| {
                        post_content.set(input_event.value().clone());
                    }
                },
                button {
                    r#type: "submit",
                    "Save"
                }
            }
        }
    }
}
