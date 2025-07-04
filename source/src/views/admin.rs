use crate::components::{AdminLogin, AdminView};
use dioxus::{
    logger::tracing::{error, info, warn},
    prelude::*,
};

// TODO: obviously this is not secure, I'll be coming back to this later
// and adding proper auth and session managment later
// but don't need to worry about that when I have no actual functionality
// TODO: Should also log bad login attempts as well
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    if username == "admin" && password == "password" {
        info!("Admin login successful");
        Ok(true)
    } else {
        warn!("Admin login failed for user: {}", username);
        Ok(false)
    }
}

// TODO: This is a placeholder for session validation logic.
// In a real application, you would check if the user is logged in.
#[server]
pub async fn validate_session() -> Result<bool, ServerFnError> {
    Ok(true)
}

#[component]
pub fn Admin() -> Element {
    let session_valid: Option<bool> = Some(true);
    //BUG: This be not working, just set to true for now and figure this part out later
    // use_server_future(|| async {
    //     session_valid = Some(validate_session().await.unwrap());
    // });

    match session_valid {
        Some(session_valid) => {
            // if session isn't valid, return to login page
            if !session_valid {
                warn!("Admin session is not valid, redirecting to login page");
                return rsx! { AdminLogin {} };
            }

            info!("Admin session is valid, rendering admin view");
            return rsx! { AdminView {} };
        }
        None => {
            // If we haven't validated yet display "loading"
            // message
            return rsx! {
                p { "Validating admin session . . ." }
            };
        }
    };
}
