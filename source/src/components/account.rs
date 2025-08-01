use crate::models::save_new_account;
use crate::models::Role;
use dioxus::logger::tracing::{error, info, warn};
use dioxus::prelude::*;

#[component]
pub fn AddAccount() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut role = use_signal(|| Role::Guest );

    rsx! {
        form {
            id: "admin-add-account-form",
            style: "display:flex; flex-direction:column;",
            onsubmit: move |_| {
                spawn(async move {
                    let username_str = username.read().to_string();
                    let password_str = password.read().to_string();

                    let mut is_error = false;
                    if username_str.is_empty() {
                        is_error = true;
                        error!("Cannot have empty username");
                    }
                    if password_str.is_empty() {
                        is_error = true;
                        error!("Cannot have empty password");
                    }
                    if is_error {
                        return;
                    }

                    let role_value = (*role.read()).clone();
                    match save_new_account(username_str, password_str, role_value).await {
                        Ok(new_account) => {
                            info!("New account created:{:?}", new_account);
                        },
                        Err(err) => {
                            error!("Error ocurred during account creation:{}", err);
                        }
                    }
                });
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
                r#type: "password",  // Changed from "text" to "password"
                placeholder: "new password",
                oninput: move |input_event| {
                    password.set(input_event.value().clone());
                }
            },
            label { "Select role"},
            select {
                onchange: move |evt| {
                    let role_enum = match evt.value().clone().as_str() {
                        "admin" => { Role::Admin},
                        "user" => { Role::User },
                        _ => { Role::Guest }
                    };
                    role.set(role_enum);
                },
                option { value: "", "Select a role..." },  // Default option
                option { value: "admin", "Admin" },
                option { value: "user", "User" },
            },
            button {
                r#type: "submit",
                "create new user"
            }
        }
    }
}
