use dioxus::prelude::*;
use dioxus::logger::tracing::{info, warn, error};

#[component]
pub fn AddAccount() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {

            form {
                id: "admin-add-account-form",
                style: "display:flex; flex-direction:column;",
                onsubmit: move |_| {
                    info!("adding new username + password to account db");
                },
                label {"Enter new username" },
                input {
                    r#type: "text",
                    placeholder: "new username",
                    oninput: move |input_event| {
                        username.set(input_event.value().clone());
                    }
                },
                label {"Enter new password" },
                input {
                    r#type: "text",
                    placeholder: "new password",
                    oninput: move |input_event| {
                        password.set(input_event.value().clone());
                    }
                }
            }
    }
}
