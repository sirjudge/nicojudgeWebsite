use crate::{
    components::{AdminLogin, AdminView},
    auth::{validate_session, validate_login}
};
use dioxus::{
    logger::tracing::{info, warn},
    prelude::*,
};



#[component]
pub fn NotWorkingNotice() -> Element {
    rsx! {
        p  { "This page does not yet have working logic and the button does nothing. More features will be implemented as time goes forth" }
    }
}

#[component]
pub fn Admin() -> Element {
    let session_valid: Option<bool> = Some(true);

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
                    NotWorkingNotice {}
                    AdminLogin {}
                };
            }

            info!("Admin session is valid, rendering admin view");
            return rsx! {
                NotWorkingNotice {}
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
