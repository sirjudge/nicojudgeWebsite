use dioxus::{
    logger::tracing::{error, info, warn},
    prelude::*,
};

// TODO: This is a placeholder for session validation logic.
// In a real application, you would check if the user is logged in.
#[server]
pub async fn validate_session() -> Result<bool, ServerFnError> {
    Ok(true)
}

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
