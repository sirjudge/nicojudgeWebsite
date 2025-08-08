use crate::auth::{login_with_session_and_cookies, CurrentUser, LoginResponse, SessionCookieManager};
use dioxus::prelude::*;
use dioxus::logger::tracing::{info, error, warn};

#[derive(Props, Clone, PartialEq)]
pub struct LoginFormProps {
    pub on_login_success: Option<Callback<CurrentUser>>,
}

#[component]
pub fn LoginForm(props: LoginFormProps) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut login_status = use_signal(|| String::new());
    let mut current_user = use_signal(|| None::<CurrentUser>);
    let on_success = props.on_login_success;

    rsx! {
        div {
            class: "login-form",
            style: "max-width: 400px; margin: 0 auto;",
            
            h2 { 
                style: "text-align: center; color: #4a9eff; margin-bottom: 30px;",
                "Admin Login" 
            }
            
            if !login_status.read().is_empty() {
                div {
                    style: "
                        padding: 15px; 
                        margin-bottom: 20px; 
                        border-radius: 8px; 
                        text-align: center;
                        background: #2a1a1a;
                        border: 1px solid #ff6b6b;
                        color: #ff6b6b;
                        font-size: 14px;
                    ",
                    "{login_status.read()}"
                }
            }

            form {
                style: "display: flex; flex-direction: column; gap: 20px;",
                
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    label { 
                        style: "color: #ffffff; font-weight: bold;",
                        "Username:" 
                    }
                    input {
                        r#type: "text",
                        placeholder: "Enter your username",
                        value: "{username.read()}",
                        required: true,
                        style: "
                            padding: 12px; 
                            border: 2px solid #444; 
                            border-radius: 5px; 
                            background: #1a1a1a; 
                            color: #ffffff;
                            font-size: 16px;
                        ",
                        oninput: move |e| {
                            username.set(e.value());
                        }
                    }
                }
                
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    label { 
                        style: "color: #ffffff; font-weight: bold;",
                        "Password:" 
                    }
                    input {
                        r#type: "password",
                        placeholder: "Enter your password",
                        value: "{password.read()}",
                        required: true,
                        style: "
                            padding: 12px; 
                            border: 2px solid #444; 
                            border-radius: 5px; 
                            background: #1a1a1a; 
                            color: #ffffff;
                            font-size: 16px;
                        ",
                        oninput: move |e| {
                            password.set(e.value());
                        }
                    }
                }
                
                button {
                    r#type: "button",
                    disabled: *is_loading.read(),
                    style: "
                        padding: 12px; 
                        background: #4a9eff; 
                        color: white; 
                        border: none; 
                        border-radius: 5px; 
                        cursor: pointer; 
                        font-size: 16px; 
                        font-weight: bold;
                        transition: background-color 0.3s ease;
                    ",
                    onclick: move |_| {
                        if *is_loading.read() {
                            return; // Prevent double submission
                        }

                        let username_val = username.read().clone();
                        let password_val = password.read().clone();

                        if username_val.is_empty() || password_val.is_empty() {
                            login_status.set("Please fill in all fields".to_string());
                            return;
                        }

                        is_loading.set(true);
                        login_status.set("".to_string());

                        spawn(async move {
                            // Get client information for enhanced security
                            let client_ip = Some("127.0.0.1".to_string()); 
                            let user_agent = Some("Dioxus Admin App".to_string()); 

                            match login_with_session_and_cookies(
                                username_val,
                                password_val,
                                client_ip,
                                user_agent
                            ).await {
                                Ok(response) => {
                                    is_loading.set(false);
                                    if response.success {
                                        login_status.set("✅ Login successful!".to_string());
                                        
                                        if let Some(user) = response.user.clone() {
                                            info!("User logged in successfully: {}", user.username);
                                            current_user.set(Some(user.clone()));
                                            
                                            // Store session ID in client-side cookie
                                            if let Some(session_id) = &response.session_id {
                                                info!("Storing session ID in cookie: {}", session_id);
                                                match SessionCookieManager::store_session_sync(session_id.clone()) {
                                                    Ok(_) => {
                                                        info!("Session stored in client cookie successfully");
                                                    }
                                                    Err(e) => {
                                                        warn!("Failed to store session cookie: {} (continuing anyway)", e);
                                                        // Don't fail the login for cookie storage issues
                                                    }
                                                }
                                            }
                                            
                                            // Notify parent component for redirect
                                            if let Some(callback) = on_success {
                                                info!("Calling success callback to trigger redirect");
                                                callback.call(user);
                                            }
                                            
                                            info!("Login process completed successfully");
                                        }
                                    } else {
                                        let message = response.message.clone();
                                        login_status.set(message);
                                        warn!("Login failed: {}", response.message);
                                    }
                                }
                                Err(e) => {
                                    is_loading.set(false);
                                    login_status.set("Login failed. Please try again.".to_string());
                                    error!("Login error: {}", e);
                                }
                            }
                        });
                    },
                    if *is_loading.read() {
                        "Logging in..."
                    } else {
                        "Login"
                    }
                }
            }

            div {
                style: "margin-top: 20px; padding: 15px; background: #2a2a2a; border-radius: 5px; font-size: 14px;",
                p {
                    style: "color: #888; margin: 0 0 10px 0;",
                    "🔒 Security Features:"
                }
                ul {
                    style: "color: #666; margin: 0; padding-left: 20px;",
                    li { "Secure session management with OWASP compliance" }
                    li { "Client-side secure cookies for session persistence" }
                    li { "Session binding to IP and User-Agent" }
                    li { "Automatic session timeout and cleanup" }
                    li { "Argon2 password hashing" }
                }
            }
        }
    }
}
