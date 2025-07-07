use dioxus::prelude::*;

// /// new/Edit blog form data
// #[derive(Default)]
// struct NewEditFormData {
//     email: String,
//     password: String,
// }
//
// #[derive(Default)]
// struct NewEditFormErrors {
//     email: Option<String>,
//     password: Option<String>,
// }

//TODO: Can find an example of form validation in dioxus here:
//https://github.com/DioxusLabs/dioxus/blob/main/examples/form.rs
#[component]
pub fn NewEditBlog() -> Element {
    rsx! {
            div {
                class: "new-edit-blog",
                h1 { "New/Edit Blog Post" }
                form {
                    label {
                        "Post Title:"
                    }
                    br { }
                    input {
                        r#type: "text",
                        placeholder: "Title",
                        name: "title",
                        required: true,
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
