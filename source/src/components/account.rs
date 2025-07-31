use dioxus::prelude::*;
use dioxus::logger::tracing::{info, warn, error};
use crate::models::save_new_account;
use crate::models::Role;

#[component]
pub fn AddAccount() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    //TODO: Change this to be Role, string is what
    //the example uses
    let mut role = use_signal(|| "".to_string());

    rsx! {

            form {
                id: "admin-add-account-form",
                style: "display:flex; flex-direction:column;",
                onsubmit: move |_| {
                    info!("adding new username + password to account db");
                    spawn(async move {
                        let username_str = username.read().to_string();
                        let password_str = username.read().to_string();
                        let role_string = role.read().to_string();
                        let role_enum = match role_string.as_str() {
                            "admin" => { Role::Admin},
                            "user" => { Role::User },
                            _ => { Role::Guest }
                        };
                        match save_new_account(username_str, password_str, role_enum).await {
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
                    r#type: "text",
                    placeholder: "new password",
                    oninput: move |input_event| {
                        password.set(input_event.value().clone());
                    }
                },
                label { "Select role"},
                select {
                    onchange: move |evt| {
                        role.set(evt.value().clone());
                    },
                    option { value: "admin", "Admin" },
                    option { value: "user", "User" },
                }
            }
    }
}
