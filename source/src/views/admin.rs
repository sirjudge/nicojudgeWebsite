use dioxus::prelude::*;
use crate::components::{
    AdminLogin,
    AdminView,
};


// TODO: obviously this is not secure, I'll be coming back to this later
// and adding proper auth and session managment later
// but don't need to worry about that when I have no actual functionality
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    if username == "admin" && password == "password" {
        Ok(true)
    } else {
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
    //TODO: This be not working, just set to true for now and figure this part out later
    // use_server_future(|| async {
    //     session_valid = Some(validate_session().await.unwrap());
    // });

    match session_valid {
        Some(session_valid) => {
            // if session isn't valid, return to login page
            if !session_valid {
                return rsx! { AdminView {} };
            }

            return rsx! { AdminLogin {} };
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

