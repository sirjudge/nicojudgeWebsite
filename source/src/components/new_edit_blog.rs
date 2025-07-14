use crate::models::{BlogPost, BlogPostModel};
use dioxus::{html::textarea, prelude::*};
use std::default;


#[derive(Debug)]
struct BlogPostFormData {
    title: String,
    content: String,
}

impl BlogPostFormData {
    fn new() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
        }
    }
}


#[server]
pub async fn save_blog_post() -> Result<BlogPost, ServerFnError> {
    let model = BlogPost {
        id: None,
        title: "Test title".to_string(),
        content: "test content".to_string(),
    };

    Ok(model)
}

//TODO: Can find an example of form validation in dioxus here:
//https://github.com/DioxusLabs/dioxus/blob/main/examples/form.rs
#[component]
pub fn NewEditBlog() -> Element {
    let mut form_data = use_signal(|| BlogPostFormData::new);
    let mut content_text_area = use_signal(|| String::new);
    rsx! {
        div {
            class: "new-edit-blog",
            h1 { "New/Edit Blog Post" }
            form {
                label {
                    "Post Title:"
                }
                input {
                    r#type: "text",
                    placeholder: "Title",
                    name: "title",
                    required: true
                }
                br { }
                label {
                    "Post Content:"
                }
                br { }
                textarea {
                    placeholder: "Content",
                    name: "content",
                    required: true,
                }
                br { }
                button {
                    r#type: "submit",
                    "Save"
                }
            }
        }
    }
}
