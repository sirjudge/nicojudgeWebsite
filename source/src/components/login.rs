use crate::auth::{login_with_session, LoginResponse, CurrentUser};
use dioxus::logger::tracing::{error, info, warn};
use dioxus::prelude::*;

#[component]
pub fn LoginForm() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut login_status = use_signal(|| "".to_string());
    let mut is_loading = use_signal(|| false);
    let mut current_user = use_signal(|| None::<CurrentUser>);

    rsx! {
        div {
            class: "login-container",
            style: "max-width: 400px; margin: 0 auto; padding: 20px;",

            h2 { "Login" }

            if !login_status.read().is_empty() {
                div {
                    class: if login_status.read().contains("successful") { "success-message" } else { "error-message" },
                    style: if login_status.read().contains("successful") {
                        "color: green; margin-bottom: 10px;"
                    } else {
                        "color: red; margin-bottom: 10px;"
                    },
                    "{login_status}"
                }
            }

            if let Some(user) = current_user.read().as_ref() {
                div {
                    class: "user-info",
                    style: "background: #f0f0f0; padding: 10px; border-radius: 5px; margin-bottom: 20px;",
                    h3 { "Welcome, {user.username}!" }
                    p { "Account ID: {user.account_id}" }
                    p { "Role ID: {user.role_id}" }
                    p { "Session ID: {user.session_id}" }
                    button {
                        onclick: move |_| {
                            current_user.set(None);
                            login_status.set("Logged out".to_string());
                        },
                        "Logout"
                    }
                }
            } else {
                form {
                    onsubmit: move |_| {
                        let username_val = username.read().clone();
                        let password_val = password.read().clone();

                        if username_val.is_empty() || password_val.is_empty() {
                            login_status.set("Please enter both username and password".to_string());
                            return;
                        }

                        is_loading.set(true);
                        login_status.set("".to_string());

                        spawn(async move {
                            match login_with_session(
                                username_val,
                                password_val,
                                Some("127.0.0.1".to_string()), // In a real app, get from request
                                Some("Dioxus App".to_string())  // In a real app, get from request headers
                            ).await {
                                Ok(response) => {
                                    is_loading.set(false);
                                    if response.success {
                                        login_status.set("Login successful!".to_string());
                                        current_user.set(response.user);
                                        info!("User logged in successfully");
                                    } else {
                                        let message = response.message.clone();
                                        login_status.set(response.message);
                                        error!("Login failed: {}", message);
                                    }
                                }
                                Err(e) => {
                                    is_loading.set(false);
                                    login_status.set(format!("Login error: {}", e));
                                    error!("Login error: {}", e);
                                }
                            }
                        });
                    },

                    div {
                        style: "margin-bottom: 15px;",
                        label {
                            style: "display: block; margin-bottom: 5px;",
                            "Username:"
                        }
                        input {
                            r#type: "text",
                            placeholder: "Enter username",
                            value: "{username}",
                            style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            oninput: move |event| {
                                username.set(event.value().clone());
                            }
                        }
                    }

                    div {
                        style: "margin-bottom: 15px;",
                        label {
                            style: "display: block; margin-bottom: 5px;",
                            "Password:"
                        }
                        input {
                            r#type: "password",
                            placeholder: "Enter password",
                            value: "{password}",
                            style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            oninput: move |event| {
                                password.set(event.value().clone());
                            }
                        }
                    }

                    button {
                        r#type: "submit",
                        disabled: *is_loading.read(),
                        style: "width: 100%; padding: 10px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        if *is_loading.read() { "Logging in..." } else { "Login" }
                    }
                }
            }
        }
    }
}
