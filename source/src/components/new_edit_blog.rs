use dioxus::prelude::*;

#[component]
pub fn NewEditBlog() -> Element {
    rsx! {
        div {
            class: "new-edit-blog",
            h1 { "New/Edit Blog Post" }
            form {
                input {
                    r#type: "text",
                    placeholder: "Title",
                    name: "title",
                    required: true,
                }
                textarea {
                    placeholder: "Content",
                    name: "content",
                    required: true,
                }
                button {
                    r#type: "submit",
                    "Save"
                }
            }
        }
    }
}
