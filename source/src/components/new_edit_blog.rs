use crate::models::{save_post, BlogPost};
use dioxus::{
    logger::tracing::{debug, error, info, warn},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlogPostFormData {
    pub title: String,
    pub content: String,
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
                onsubmit:  move |_| {
                    //TODO: Figure out how to make the onsubmit an async method but for now just
                    //use spawn
                    spawn(async move {
                        let form_data = BlogPostFormData {
                            title: post_title.read().to_string(),
                            content: post_content.read().to_string()
                        };

                        let blog_post = BlogPost::from_form_data(form_data);
                        match save_post(blog_post).await {
                            Ok(saved_post) => {
                                info!("saved new post: {:?}", saved_post);
                                //TODO: Should redirect to the view blog post page on save or have
                                //a pop up that will take the user there if clicked
                            }
                            Err(e) => {
                                error!("Failed to save post: {}", e);
                                //TODO: Should display an error message/prompt here
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
