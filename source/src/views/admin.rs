use crate::{
    auth::{get_current_user, logout_with_cookies, CurrentUser, SessionState, SessionCookieManager},
    components::{AdminView, LoginForm},
    views::Navbar,
};
use dioxus::{
    logger::tracing::{info, warn, error},
    prelude::*,
};

#[component]
pub fn Admin() -> Element {
    let mut session_state = use_signal(|| SessionState::Loading);
    let mut current_user = use_signal(|| None::<CurrentUser>);

    // Check for session on component mount using client-side cookie validation
    use_effect(move || {
        spawn(async move {
            // Try to get session ID from client-side cookies
            match SessionCookieManager::get_session_id_sync() {
                Some(session_id) => {
                    info!("Session ID found in client cookie: {}", session_id);
                    
                    // Validate session with server
                    match get_current_user(session_id.clone()).await {
                        Ok(Some(user)) => {
                            info!("Valid session found for user: {:?}", user);
                            current_user.set(Some(user));
                            session_state.set(SessionState::Valid);
                        }
                        Ok(None) => {
                            warn!("Session not found or expired");
                            SessionCookieManager::clear_session_sync();
                            session_state.set(SessionState::Invalid);
                        }
                        Err(e) => {
                            error!("Session validation failed: {}", e);
                            // Clear invalid session cookie
                            SessionCookieManager::clear_session_sync();
                            session_state.set(SessionState::Invalid);
                        }
                    }
                }
                None => {
                    info!("No session ID found in client cookies");
                    session_state.set(SessionState::Invalid);
                }
            }
        });
    });

    rsx! {
        Navbar {}
        div {
            style: "padding: 20px;",
            
            match session_state.read().clone() {
                SessionState::Loading => rsx! {
                    div {
                        style: "text-align: center; padding: 40px; color: #888;",
                        p { "Checking authentication..." }
                    }
                },
                SessionState::Valid => {
                    let user_info = current_user.read().clone();
                    rsx! {
                        div {
                            style: "margin-bottom: 20px; padding: 15px; background: #2a2a2a; border-radius: 8px;",
                            h2 { 
                                style: "color: #4a9eff; margin: 0 0 10px 0;",
                                "Admin Panel" 
                            }
                            if let Some(user) = user_info {
                                p { 
                                    style: "color: #888; margin: 5px 0;",
                                    "Welcome, {user.username}!" 
                                }
                                p { 
                                    style: "color: #666; margin: 5px 0; font-size: 14px;",
                                    "Role ID: {user.role_id}" 
                                }
                            }
                            button {
                                style: "
                                    background: #ff6b6b; 
                                    color: white; 
                                    border: none; 
                                    padding: 8px 16px; 
                                    border-radius: 4px; 
                                    cursor: pointer;
                                    margin-top: 10px;
                                ",
                                onclick: move |_| {
                                    let mut session_state_clone = session_state.clone();
                                    let mut current_user_clone = current_user.clone();
                                    
                                    spawn(async move {
                                        // Clear client-side session cookie first
                                        let session_id = SessionCookieManager::get_session_id_sync();
                                        SessionCookieManager::clear_session_sync();
                                        
                                        // Call server logout if we have a session ID
                                        if let Some(sid) = session_id {
                                            if let Err(e) = logout_with_cookies(sid).await {
                                                error!("Logout error: {}", e);
                                            }
                                        }
                                        
                                        // Update UI state
                                        session_state_clone.set(SessionState::Invalid);
                                        current_user_clone.set(None);
                                    });
                                },
                                "Logout"
                            }
                        }
                        AdminView {}
                    }
                },
                SessionState::Invalid => {
                    warn!("Invalid or no session, showing login form");
                    rsx! {
                        div {
                            style: "max-width: 400px; margin: 0 auto;",
                            h2 { 
                                style: "color: #4a9eff; text-align: center; margin-bottom: 20px;",
                                "Admin Login" 
                            }
                            LoginForm {
                                on_login_success: move |_| {
                                    info!("Login successful, updating session state");
                                    session_state.set(SessionState::Loading);
                                    
                                    // Re-check session after successful login
                                    spawn(async move {
                                        // Get session ID from cookie after successful login
                                        if let Some(session_id) = SessionCookieManager::get_session_id_sync() {
                                            match get_current_user(session_id).await {
                                                Ok(Some(user)) => {
                                                    info!("User authenticated after login: {:?}", user);
                                                    current_user.set(Some(user));
                                                    session_state.set(SessionState::Valid);
                                                }
                                                Ok(None) => {
                                                    error!("No user found after successful login");
                                                    session_state.set(SessionState::Invalid);
                                                }
                                                Err(e) => {
                                                    error!("Failed to get user after login: {}", e);
                                                    session_state.set(SessionState::Invalid);
                                                }
                                            }
                                        } else {
                                            error!("No session ID found after successful login");
                                            session_state.set(SessionState::Invalid);
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
