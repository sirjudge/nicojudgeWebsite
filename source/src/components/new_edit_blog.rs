use crate::models::{save_post, BlogPost};

use dioxus::{
    html::textarea,
    logger::tracing::{debug, error, info, warn},
    prelude::{server_fn::{error::Result}, *}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlogPostFormData {
    pub title: String,
    pub content: String,
}

#[server]
pub async fn save_new_post(blog_form_data: BlogPostFormData) -> Result<Option<BlogPost>, ServerFnError> {
    let blog_post = BlogPost::from_form_data(blog_form_data);
    info!("Attempting to save blog post:{}", blog_post.title);
    save_post(blog_post).await
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
                    let form_data = BlogPostFormData {
                        title: post_title.read().to_string(),
                        content: post_content.read().to_string()
                    };

                    spawn(async move {
                        match save_new_post(form_data).await {
                            Ok(saved_post) => {
                                info!("saved new post: {:?}", saved_post);
                            }
                            Err(e) => {
                                error!("Failed to save post: {}", e);
                            }
                        }
                    });
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
