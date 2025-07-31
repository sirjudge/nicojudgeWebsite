use crate::{
    auth::{validate_login, validate_session},
    components::{AdminLogin, AdminView},
};
use dioxus::{
    logger::tracing::{info, warn},
    prelude::*,
};

#[component]
pub fn Admin() -> Element {
    let session_valid: Option<bool> = Some(false);

    //TODO: Swap back to use this to validate a session first then
    // display admin only if proper session validation has ocurred
    // which means a successful login has ocurred
    // let session_valid: Option<bool> = Some(false);
    //BUG: This be not working, just set to true for now and figure this part out later
    // use_server_future(|| async {
    //     session_valid = Some(validate_session().await.unwrap());
    // });

    match session_valid {
        Some(session_valid) => {
            // if session isn't valid, return to login page
            if !session_valid {
                warn!("Admin session is not valid, redirecting to login page");
                return rsx! {
                    AdminLogin {}
                };
            }

            info!("Admin session is valid, rendering admin view");
            return rsx! {
                AdminView {}
            };
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
