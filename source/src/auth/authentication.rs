#[cfg(feature = "server")]
use crate::database::create_connection;
use crate::{
    auth::{create_session, get_session_with_validation, Session, SessionCookieManager, invalidate_session},
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

/// Simple test server function to verify server functions work
#[server]
pub async fn test_server_function() -> Result<String, ServerFnError> {
    Ok("Server function is working!".to_string())
}

/// Simple test login function to debug network issues
#[server]
pub async fn test_login_simple(username: String, password: String) -> Result<String, ServerFnError> {
    info!("Test login called with username: {}", username);
    
    // Test database connection first
    match crate::database::create_connection().await {
        Ok(_) => {
            info!("Database connection successful");
            Ok(format!("Database connection works for user: {}", username))
        }
        Err(e) => {
            error!("Database connection failed: {}", e);
            Err(ServerFnError::new(format!("Database connection failed: {}", e)))
        }
    }
}

/// Minimal login test that checks account lookup only
#[server]
pub async fn test_login_account_lookup(username: String) -> Result<String, ServerFnError> {
    info!("Testing account lookup for username: {}", username);
    
    match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("Account found: {}", account.username);
            Ok(format!("Account found: {} with role_id: {}", account.username, account.role_id))
        }
        Ok(None) => {
            info!("No account found for username: {}", username);
            Ok(format!("No account found for username: {}", username))
        }
        Err(e) => {
            error!("Error during account lookup: {}", e);
            Err(ServerFnError::new(format!("Account lookup error: {}", e)))
        }
    }
}

/// Test password verification only
#[server]
pub async fn test_password_verify(password: String, stored_hash: String) -> Result<String, ServerFnError> {
    info!("Testing password verification");
    
    match verify_password_hash(password, stored_hash).await {
        Ok(is_valid) => {
            info!("Password verification result: {}", is_valid);
            Ok(format!("Password verification: {}", is_valid))
        }
        Err(e) => {
            error!("Password verification error: {}", e);
            Err(e)
        }
    }
}

/// Simplified login without session creation - for debugging NS_BINDING_ABORTED
#[server]
pub async fn test_login_no_session(
    username: String,
    password: String,
) -> Result<LoginResponse, ServerFnError> {
    info!("Simple login test for user: {}", username);
    
    // Step 1: Database connection test
    info!("Step 1: Testing database connection...");
    let _conn_test = match crate::database::create_connection().await {
        Ok(_) => {
            info!("✅ Database connection successful");
        }
        Err(e) => {
            error!("❌ Database connection failed: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Database connection failed: {}", e),
                session_id: None,
                user: None,
            });
        }
    };
    
    // Step 2: Account lookup
    info!("Step 2: Looking up account...");
    let account = match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("✅ Account found: {}", account.username);
            account
        }
        Ok(None) => {
            info!("❌ No account found for username: {}", username);
            return Ok(LoginResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                session_id: None,
                user: None,
            });
        }
        Err(e) => {
            error!("❌ Account lookup error: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Account lookup error: {}", e),
                session_id: None,
                user: None,
            });
        }
    };
    
    // Step 3: Password verification
    info!("Step 3: Verifying password...");
    let is_valid = match verify_password_hash(password, account.password_hash).await {
        Ok(valid) => {
            info!("✅ Password verification result: {}", valid);
            valid
        }
        Err(e) => {
            error!("❌ Password verification failed: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Password verification error: {}", e),
                session_id: None,
                user: None,
            });
        }
    };
    
    if is_valid {
        info!("✅ Login successful (without session)");
        Ok(LoginResponse {
            success: true,
            message: "Login successful (no session created)".to_string(),
            session_id: None, // No session for this test
            user: Some(CurrentUser {
                account_id: account.account_id.unwrap_or(0),
                username: account.username,
                role_id: account.role_id,
                session_id: "test-session".to_string(), // Dummy session ID
            }),
        })
    } else {
        info!("❌ Invalid password");
        Ok(LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            session_id: None,
            user: None,
        })
    }
}

/// Test session creation only - to isolate NS_BINDING_ABORTED issue
#[server]
pub async fn test_session_creation(account_id: i32) -> Result<String, ServerFnError> {
    info!("Testing session creation for account_id: {}", account_id);
    
    match create_session(
        account_id,
        Some("127.0.0.1".to_string()),
        Some("Test User Agent".to_string())
    ).await {
        Ok(session) => {
            info!("✅ Session created successfully: {}", session.session_id);
            Ok(format!("Session created: {} (expires: {:?})", session.session_id, session.expires_at))
        }
        Err(e) => {
            error!("❌ Session creation failed: {}", e);
            Err(ServerFnError::new(format!("Session creation failed: {}", e)))
        }
    }
}

/// Simplified session creation without concurrent session limits - for debugging
#[server]
pub async fn test_simple_session_creation(account_id: i32) -> Result<String, ServerFnError> {
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    
    info!("Testing simple session creation for account_id: {}", account_id);
    
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);
    let ip_address = Some("127.0.0.1".to_string());
    let user_agent = Some("Test User Agent".to_string());
    
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
                    info!("✅ Simple session created successfully: {}", session_id);
                    Ok(format!("Simple session created: {} (expires: {:?})", session_id, expires_at))
                }
                Err(e) => {
                    error!("❌ Simple session creation failed: {}", e);
                    Err(ServerFnError::new(format!("Simple session creation failed: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("❌ Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

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

/// Enhanced login function with secure cookie management
#[server]
pub async fn login_with_session_and_cookies(
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    info!("Login attempt for user: {} from IP: {:?}", username, ip_address);
    
    // Test database connection first
    info!("Testing database connection...");
    match crate::database::create_connection().await {
        Ok(_) => info!("Database connection successful"),
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Err(ServerFnError::new(format!("Database connection failed: {}", e)));
        }
    }
    
    info!("Looking up account by username: {}", username);
    match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("Account found for user: {}", account.username);
            // Verify the password
            info!("Verifying password...");
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
                // Create a new session with enhanced security context
                let session = match create_session(
                    account.account_id.unwrap_or(0), 
                    ip_address.clone(), 
                    user_agent.clone()
                ).await {
                    Ok(session) => {
                        info!("Session created successfully: {}", session.session_id);
                        session
                    }
                    Err(e) => {
                        error!("Session creation failed: {}", e);
                        return Err(e);
                    }
                };

                info!("Login successful for user: {} from IP: {:?}", account.username, ip_address);

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

/// Login function that creates a session (legacy compatibility)
#[server]
pub async fn login_with_session(
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    login_with_session_and_cookies(username, password, ip_address, user_agent).await
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

/// Test version of login with simplified session creation - for debugging NS_BINDING_ABORTED
#[server]
pub async fn test_login_with_simple_session(
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    info!("Test login with simple session for user: {} from IP: {:?}", username, ip_address);
    
    // Step 1: Database connection test
    info!("Step 1: Testing database connection...");
    match crate::database::create_connection().await {
        Ok(_) => info!("✅ Database connection successful"),
        Err(e) => {
            error!("❌ Database connection failed: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Database connection failed: {}", e),
                session_id: None,
                user: None,
            });
        }
    }
    
    // Step 2: Account lookup
    info!("Step 2: Looking up account...");
    let account = match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("✅ Account found: {}", account.username);
            account
        }
        Ok(None) => {
            info!("❌ No account found for username: {}", username);
            return Ok(LoginResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                session_id: None,
                user: None,
            });
        }
        Err(e) => {
            error!("❌ Account lookup error: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Account lookup error: {}", e),
                session_id: None,
                user: None,
            });
        }
    };
    
    // Step 3: Password verification
    info!("Step 3: Verifying password...");
    let is_valid = match verify_password_hash(password, account.password_hash).await {
        Ok(valid) => {
            info!("✅ Password verification result: {}", valid);
            valid
        }
        Err(e) => {
            error!("❌ Password verification failed: {}", e);
            return Ok(LoginResponse {
                success: false,
                message: format!("Password verification error: {}", e),
                session_id: None,
                user: None,
            });
        }
    };
    
    if !is_valid {
        info!("❌ Invalid password");
        return Ok(LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            session_id: None,
            user: None,
        });
    }
    
    // Step 4: Create session using simplified method
    info!("Step 4: Creating session with simplified method...");
    let session_result = test_simple_session_creation(account.account_id.unwrap_or(0)).await;
    
    match session_result {
        Ok(session_info) => {
            info!("✅ Session created successfully: {}", session_info);
            
            // Extract session ID from the response (it's in the format "Simple session created: {id} ...")
            let session_id = if let Some(start) = session_info.find("Simple session created: ") {
                let id_start = start + "Simple session created: ".len();
                if let Some(end) = session_info[id_start..].find(" ") {
                    session_info[id_start..id_start + end].to_string()
                } else {
                    "test-session-id".to_string()
                }
            } else {
                "test-session-id".to_string()
            };
            
            Ok(LoginResponse {
                success: true,
                message: "Login successful with simplified session".to_string(),
                session_id: Some(session_id.clone()),
                user: Some(CurrentUser {
                    account_id: account.account_id.unwrap_or(0),
                    username: account.username,
                    role_id: account.role_id,
                    session_id,
                }),
            })
        }
        Err(e) => {
            error!("❌ Session creation failed: {}", e);
            Ok(LoginResponse {
                success: false,
                message: format!("Session creation failed: {}", e),
                session_id: None,
                user: None,
            })
        }
    }
}

/// Fixed login function with working session creation - replaces the problematic one
#[server]
pub async fn login_with_session_and_cookies_fixed(
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    info!("Fixed login attempt for user: {} from IP: {:?}", username, ip_address);
    
    // Test database connection first
    info!("Testing database connection...");
    match crate::database::create_connection().await {
        Ok(_) => info!("Database connection successful"),
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Err(ServerFnError::new(format!("Database connection failed: {}", e)));
        }
    }
    
    info!("Looking up account by username: {}", username);
    match get_account_by_username(username.clone()).await {
        Ok(Some(account)) => {
            info!("Account found for user: {}", account.username);
            // Verify the password
            info!("Verifying password...");
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
                info!("Password valid, creating session with simplified method...");
                
                // Use the working simplified session creation instead of the problematic create_session
                let account_id = account.account_id.unwrap_or(0);
                let session_result = test_simple_session_creation(account_id).await;
                
                match session_result {
                    Ok(session_info) => {
                        info!("Session created successfully with simplified method: {}", session_info);
                        
                        // Extract session ID from the response
                        let session_id = if let Some(start) = session_info.find("Simple session created: ") {
                            let id_start = start + "Simple session created: ".len();
                            if let Some(end) = session_info[id_start..].find(" ") {
                                session_info[id_start..id_start + end].to_string()
                            } else {
                                format!("session-{}", account_id)
                            }
                        } else {
                            format!("session-{}", account_id)
                        };
                        
                        info!("Extracted session ID: {}", session_id);

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
                    }
                    Err(e) => {
                        error!("Session creation failed: {}", e);
                        return Err(e);
                    }
                }
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
