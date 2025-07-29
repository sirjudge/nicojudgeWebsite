use dioxus::{
    logger::tracing::{info,warn},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};
use crate::models::{Account, get_account_by_username};

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

    let account = get_account_by_username(username);

    //TODO: Final step should be to remove this
    Ok(true)
}

#[server]
async fn hash_password(password: String) -> Result<String, ServerFnError> {
    if password.is_empty() {
        return Err(ServerFnError::new("input password is empty when it should not be"));
    }


    Ok("".to_string())
}
