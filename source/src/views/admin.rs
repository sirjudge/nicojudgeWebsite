use crate::{
    auth::{get_session, get_current_user, CurrentUser},
    components::{AdminView, LoginForm},
};
use dioxus::{
    logger::tracing::{info, warn, error},
    prelude::*,
};

#[component]
pub fn Admin() -> Element {
    let mut session_state = use_signal(|| SessionState::Loading);
    let mut current_user = use_signal(|| None::<CurrentUser>);

    // Check for session on component mount
    use_effect(move || {
        spawn(async move {
            // Try to get session ID from local storage or a cookie
            // For now, we'll simulate this - in a real app you'd get this from browser storage
            if let Some(session_id) = get_stored_session_id().await {
                match get_current_user(session_id).await {
                    Ok(Some(user)) => {
                        info!("Valid session found for user: {}", user.username);
                        current_user.set(Some(user));
                        session_state.set(SessionState::Valid);
                    }
                    Ok(None) => {
                        warn!("Session not found or expired");
                        session_state.set(SessionState::Invalid);
                    }
                    Err(e) => {
                        error!("Error validating session: {}", e);
                        session_state.set(SessionState::Invalid);
                    }
                }
            } else {
                info!("No session ID found");
                session_state.set(SessionState::Invalid);
            }
        });
    });

    // Create separate signals to avoid borrow conflicts
    let session_state_clone = session_state.clone();
    let current_user_clone = current_user.clone();

    // Get the current session state value to avoid borrow conflicts
    let current_session_state = session_state.read().clone();
    
    match current_session_state {
        SessionState::Loading => {
            rsx! {
                div {
                    style: "text-align: center; padding: 50px;",
                    p { "Validating session..." }
                    p { 
                        style: "color: #666; font-size: 0.9em;",
                        "Please wait while we check your authentication status."
                    }
                }
            }
        }
        SessionState::Valid => {
            // Check if we have a valid user, otherwise switch to invalid state
            match current_user.read().as_ref() {
                Some(user) => {
                    let user_info = user.clone(); // Clone to avoid borrow issues
                    info!("Rendering admin view for user: {}", user_info.username);
                    rsx! {
                        div {
                            AdminView {}
                            // Add a logout option
                            div {
                                style: "position: fixed; top: 10px; right: 10px; background: white; padding: 10px; border: 1px solid #ddd; border-radius: 5px;",
                                p { 
                                    style: "margin: 0 0 10px 0; font-size: 0.9em;",
                                    "Logged in as: {user_info.username}"
                                }
                                button {
                                    onclick: move |_| {
                                        let mut session_state_ref = session_state_clone.clone();
                                        let mut current_user_ref = current_user_clone.clone();
                                        // Clear session
                                        spawn(async move {
                                            clear_stored_session_id().await;
                                            current_user_ref.set(None);
                                            session_state_ref.set(SessionState::Invalid);
                                        });
                                    },
                                    style: "background: #dc3545; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer;",
                                    "Logout"
                                }
                            }
                        }
                    }
                }
                None => {
                    // No user data but state is Valid - this is an inconsistent state
                    // Show loading while we fix the state
                    rsx! {
                        div {
                            style: "text-align: center; padding: 50px;",
                            p { "Refreshing session..." }
                        }
                    }
                }
            }
        }
        SessionState::Invalid => {
            warn!("Invalid or no session, showing login form");
            rsx! {
                div {
                    LoginForm {
                        on_login_success: move |user: CurrentUser| {
                            info!("Login successful, updating admin session state");
                            current_user.set(Some(user));
                            session_state.set(SessionState::Valid);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum SessionState {
    Loading,
    Valid,
    Invalid,
}

// Helper functions for session storage
// In a real application, these would interact with browser localStorage or cookies
async fn get_stored_session_id() -> Option<String> {
    // TODO: Implement actual session storage retrieval
    // For now, return None to simulate no stored session
    // 
    // Real implementation examples:
    // 
    // Option 1: Using web_sys for localStorage
    // use web_sys::window;
    // if let Some(window) = window() {
    //     if let Ok(Some(storage)) = window.local_storage() {
    //         return storage.get_item("session_id").ok().flatten();
    //     }
    // }
    // 
    // Option 2: Using cookies (would need a cookie crate)
    // return get_cookie("session_id");
    // 
    // Option 3: Using URL parameters or headers in SSR context
    
    warn!("Session storage not implemented - returning None");
    None
}

async fn store_session_id(session_id: String) {
    // TODO: Implement actual session storage
    // 
    // Real implementation examples:
    // 
    // Option 1: Using web_sys for localStorage
    // use web_sys::window;
    // if let Some(window) = window() {
    //     if let Ok(Some(storage)) = window.local_storage() {
    //         let _ = storage.set_item("session_id", &session_id);
    //     }
    // }
    // 
    // Option 2: Using cookies (would need a cookie crate)
    // set_cookie("session_id", &session_id, expires_in_days(30));
    
    info!("Session ID would be stored: {}", session_id);
}

async fn clear_stored_session_id() {
    // TODO: Implement actual session storage clearing
    // 
    // Real implementation examples:
    // 
    // Option 1: Using web_sys for localStorage
    // use web_sys::window;
    // if let Some(window) = window() {
    //     if let Ok(Some(storage)) = window.local_storage() {
    //         let _ = storage.remove_item("session_id");
    //     }
    // }
    // 
    // Option 2: Using cookies
    // clear_cookie("session_id");
    
    info!("Session cleared from storage");
}
