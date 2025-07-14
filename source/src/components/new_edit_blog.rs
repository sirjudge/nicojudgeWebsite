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
    let mut post_title = use_signal(|| String::new);
    let mut post_content = use_signal(|| String::new);
    let mut content_text_area = use_signal(|| String::new);
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
                    // println!("title: {:#?} content: {:#?}", title, content);
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
                        println!("Input event: {:#?}", input_event);
                        // post_title.set(input_event);
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
                        println!("Input event: {:#?}", input_event.value());
                        // post_content.set(input_event.value());
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
