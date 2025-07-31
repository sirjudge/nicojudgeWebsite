#[cfg(feature = "server")]
use crate::database::create_connection;
use crate::models::{get_account_by_username, get_account_by_id, Account, Session, create_session, get_session};
#[cfg(feature = "server")]
use password_hash::SaltString;
// #[cfg(feature = "server")]
// use rand::rngs::OsRng;
#[cfg(feature = "server")]
use argon2::{password_hash::Salt, Argon2, PasswordHasher, PasswordVerifier};
use dioxus::{
    logger::tracing::{error, info, warn},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

/// Current user information for the session
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CurrentUser {
    pub account_id: i32,
    pub username: String,
    pub role_id: i32,
    pub session_id: String,
}

/// Login request structure
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response structure
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub session_id: Option<String>,
    pub user: Option<CurrentUser>,
}

// TODO: This is a placeholder for session validation logic.
// In a real application, you would check if the user is logged in.
#[server]
pub async fn validate_session() -> Result<bool, ServerFnError> {
    warn!("This feature is not yet implemented and is returning true every time");
    Ok(true)
}

/// Get the current user from session
#[server]
pub async fn get_current_user(session_id: String) -> Result<Option<CurrentUser>, ServerFnError> {
    match get_session(session_id.clone()).await? {
        Some(session) => {
            // Get the account details by ID
            match get_account_by_id(session.account_id).await? {
                Some(account) => {
                    Ok(Some(CurrentUser {
                        account_id: account.account_id.unwrap_or(0),
                        username: account.username,
                        role_id: account.role_id,
                        session_id,
                    }))
                }
                None => Ok(None)
            }
        }
        None => Ok(None)
    }
}

/// Login function that creates a session
#[server]
pub async fn login_with_session(
    username: String, 
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>
) -> Result<LoginResponse, ServerFnError> {
    match get_account_by_username(username.clone()).await? {
        Some(account) => {
            // Verify the password
            let is_valid = verify_password_hash(password, account.password_hash).await?;
            if is_valid {
                // Create a new session
                let session = create_session(
                    account.account_id.unwrap_or(0),
                    ip_address,
                    user_agent
                ).await?;

                info!("Login successful for user: {}", account.username);
                
                Ok(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    session_id: Some(session.session_id.clone()),
                    user: Some(CurrentUser {
                        account_id: account.account_id.unwrap_or(0),
                        username: account.username,
                        role_id: account.role_id,
                        session_id: session.session_id,
                    }),
                })
            } else {
                warn!("Invalid password attempt for user: {}", username);
                Ok(LoginResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                    session_id: None,
                    user: None,
                })
            }
        }
        None => {
            warn!("Login attempt for non-existent user: {}", username);
            Ok(LoginResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                session_id: None,
                user: None,
            })
        }
    }
}

// TODO: obviously this is not secure, I'll be coming back to this later
// and adding proper auth and session managment later
// but don't need to worry about that when I have no actual functionality
// TODO: Should also log bad login attempts as well
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    match get_account_by_username(username).await? {
        Some(account) => {
            // Use the verify_password_hash function to check the password
            let is_valid = verify_password_hash(password, account.password_hash).await?;
            if is_valid {
                info!("Login successful for user: {}", account.username);
                Ok(true)
            } else {
                warn!("Invalid password attempt for user: {}", account.username);
                Ok(false)
            }
        }
        None => {
            warn!("Login attempt for non-existent user");
            Ok(false)
        }
    }
}

#[server]
pub async fn hash_password(password: String) -> Result<String, ServerFnError> {
    if password.is_empty() {
        return Err(ServerFnError::new(
            "input password is empty when it should not be",
        ));
    }

    // Use Argon2 for password hashing
    let argon2 = Argon2::default();
    
    // Generate a random salt (more secure than fixed salt)
    use password_hash::rand_core::OsRng;
    let salt = SaltString::generate(&mut OsRng);
    
    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(format!("Failed to hash password: {}", e)))?;
    
    info!("Successfully hashed password");
    
    // Return the hash string (includes the salt)
    Ok(password_hash.to_string())
}

#[server]
pub async fn verify_password_hash(password: String, stored_hash: String) -> Result<bool, ServerFnError> {
    if password.is_empty() {
        return Err(ServerFnError::new("Password cannot be empty"));
    }
    
    if stored_hash.is_empty() {
        return Err(ServerFnError::new("Stored hash cannot be empty"));
    }
    
    // Parse the stored hash
    use password_hash::PasswordHash;
    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|e| ServerFnError::new(format!("Failed to parse stored hash: {}", e)))?;
    
    // Verify the password against the stored hash
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false), // Don't expose the specific error for security reasons
    }
}
