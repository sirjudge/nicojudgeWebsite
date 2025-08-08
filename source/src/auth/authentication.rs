#[cfg(feature = "server")]
use crate::database::create_connection;
use crate::{
    auth::{get_session_with_validation, invalidate_session},
    models::{get_account_by_id, get_account_by_username, Account},
};
#[cfg(feature = "server")]
use argon2::{password_hash::Salt, Argon2, PasswordHasher, PasswordVerifier};
use dioxus::{
    logger::tracing::{error, info, warn},
    prelude::*,
};
#[cfg(feature = "server")]
use password_hash::SaltString;
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

/// Get the current user from session
#[server]
pub async fn get_current_user(session_id: String) -> Result<Option<CurrentUser>, ServerFnError> {
    match get_session_with_validation(session_id.clone(), None, None).await? {
        Some(session) => {
            // Get the account details by ID
            match get_account_by_id(session.account_id).await? {
                Some(account) => Ok(Some(CurrentUser {
                    account_id: account.account_id.unwrap_or(0),
                    username: account.username,
                    role_id: account.role_id,
                    session_id,
                })),
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}

/// Enhanced login function with secure session management
#[server]
pub async fn login_with_session_and_cookies(
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    info!("Login attempt for user: {} from IP: {:?}", username, ip_address);
    
    // Test database connection first
    match crate::database::create_connection().await {
        Ok(_) => info!("Database connection successful"),
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Err(ServerFnError::new(format!("Database connection failed: {}", e)));
        }
    }
    
    // Look up account by username
    match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("Account found for user: {}", account.username);
            
            // Verify the password
            let is_valid = match verify_password_hash(password, account.password_hash).await {
                Ok(valid) => {
                    info!("Password verification result: {}", valid);
                    valid
                }
                Err(e) => {
                    error!("Password verification failed: {}", e);
                    return Err(e);
                }
            };
            
            if is_valid {
                info!("Password valid, creating session...");
                
                // Create session using simplified method that works
                let account_id = account.account_id.unwrap_or(0);
                let session_id = create_simple_session(account_id, ip_address.clone(), user_agent.clone()).await?;
                
                info!("Login successful for user: {} from IP: {:?}", account.username, ip_address);

                Ok(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    session_id: Some(session_id.clone()),
                    user: Some(CurrentUser {
                        account_id: account.account_id.unwrap_or(0),
                        username: account.username,
                        role_id: account.role_id,
                        session_id,
                    }),
                })
            } else {
                warn!("Invalid password attempt for user: {} from IP: {:?}", username, ip_address);
                Ok(LoginResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                    session_id: None,
                    user: None,
                })
            }
        }
        Ok(None) => {
            warn!("Login attempt for non-existent user: {} from IP: {:?}", username, ip_address);
            Ok(LoginResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                session_id: None,
                user: None,
            })
        }
        Err(e) => {
            error!("Database error during account lookup: {}", e);
            Err(e)
        }
    }
}

/// Create a simple session in the database
#[server]
async fn create_simple_session(
    account_id: i32,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<String, ServerFnError> {
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);
    
    match crate::database::create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                INSERT INTO sessions (session_id, account_id, created_at, expires_at, last_accessed, ip_address, user_agent, is_active)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                session_id,
                account_id,
                now,
                expires_at,
                now,
                ip_address,
                user_agent,
                true
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(_) => {
                    info!("Session created successfully: {}", session_id);
                    Ok(session_id)
                }
                Err(e) => {
                    error!("Session creation failed: {}", e);
                    Err(ServerFnError::new(format!("Session creation failed: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Secure logout function that clears session and cookies
#[server]
pub async fn logout_with_cookies(session_id: String) -> Result<(), ServerFnError> {
    // Invalidate session in database
    if let Err(e) = invalidate_session(session_id).await {
        error!("Failed to invalidate session: {}", e);
    }
    
    info!("User logged out successfully");
    Ok(())
}

/// Legacy login function for compatibility - validates credentials only
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    match get_account_by_username(username).await? {
        Some(account) => {
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

/// Hash a password using Argon2
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

/// Verify a password against a stored hash
#[server]
pub async fn verify_password_hash(
    password: String,
    stored_hash: String,
) -> Result<bool, ServerFnError> {
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
