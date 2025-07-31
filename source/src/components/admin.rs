use crate::{
    auth::{validate_login, validate_session},
    components::{MaintenanceSettings, NewEditBlog},
    models::BlogPost,
};
use dioxus::logger::tracing::{error, info, warn};
use dioxus::prelude::{server_fn::error::Result, *};
use serde::{Deserialize, Serialize};

#[component]
pub fn AdminView() -> Element {
    // TODO: check to see if the user is verifiedly logged in, maybe do some
    // cookie magic or session management stuff
    rsx! {
        div {
            class: "admin-page",
            h1 { "Admin Page" }
            p { "This is the admin page for managing the application." }
            MaintenanceSettings {}
            NewEditBlog {}
        }
    }
}

#[component]
pub fn AdminLogin() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "admin-login",
            h1 { "Admin Login" }
            form {
                id: "admin-login-form",
                style: "display:flex; flex-direction:column;",
                onsubmit: move |_| {
                    info!("Form submitted with username:{} and password:{}", username, password);
                    spawn(async move {
                        let username_string = username.read().to_string();
                        let password_string = password.read().to_string();
                        match validate_login(username_string, password_string).await {
                            Ok(is_valid) => {
                                if is_valid {
                                    info!("Successful login yipee!");
                                }
                                else {
                                    error!("Unsuccesful error, nope")
                                }
                            }
                            Err(err) => {
                                error!("Login error:{}", err);
                            }
                        }
                    });
                },
                label { "Username:" },
                input {
                    r#type: "text",
                    placeholder: "Username",
                    name: "username",
                    required: true,
                    oninput: move |input_event| {
                        username.set(input_event.value().clone());
                    }
                },
                label { "Password:" },
                input {
                    r#type: "password",
                    placeholder: "Password",
                    name: "password",
                    required: true,
                    oninput: move |input_event| {
                        password.set(input_event.value().clone());
                    }
                },
                //TODO: pass this to a server function and validate the login
                // and redirect to admin page if successful
                button {
                    r#type: "submit",
                    "Login"
                }
            }
        }
    }
}
