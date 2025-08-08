use dioxus::prelude::{ServerFnError, *};
use dioxus::logger::tracing::{info, warn, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};
#[cfg(feature = "server")]
use uuid::Uuid;

#[derive(Clone, PartialEq)]

pub enum SessionState {
    Loading,
    Valid,
    Invalid,
}

/// Session data structure
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Session {
    pub session_id: String,
    pub account_id: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

/// Session configuration following OWASP recommendations
pub struct SessionConfig {
    pub duration_hours: i64,
    pub extend_on_access: bool,
    pub cleanup_expired: bool,
    pub bind_to_ip: bool,           // OWASP: Bind session to IP address
    pub bind_to_user_agent: bool,   // OWASP: Bind session to User-Agent
    pub max_concurrent_sessions: Option<u32>, // OWASP: Limit concurrent sessions
    pub session_timeout_minutes: i64, // OWASP: Idle timeout
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            duration_hours: 24, // 24 hours default
            extend_on_access: true,
            cleanup_expired: true,
            bind_to_ip: true,     // OWASP: Enable IP binding for security
            bind_to_user_agent: true, // OWASP: Enable User-Agent binding
            max_concurrent_sessions: Some(5), // OWASP: Limit to 5 concurrent sessions
            session_timeout_minutes: 30, // OWASP: 30-minute idle timeout
        }
    }
}

/// Enhanced session creation with security context
#[server]
pub async fn create_session(
    account_id: i32,
    ip_address: Option<String>,
    user_agent: Option<String>
) -> Result<Session, ServerFnError> {
    let config = SessionConfig::default();
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::hours(config.duration_hours);

    // OWASP: Limit concurrent sessions per user
    // TEMPORARILY DISABLED FOR DEBUGGING NS_BINDING_ABORTED ISSUE
    /*
    if let Some(max_sessions) = config.max_concurrent_sessions {
        let active_sessions = count_active_sessions(account_id).await?;
        if active_sessions >= max_sessions {
            warn!("Maximum concurrent sessions reached for account_id: {}", account_id);
            // Optionally invalidate oldest session or deny new session
            invalidate_oldest_session(account_id).await?;
        }
    }
    */
    info!("Skipping concurrent session limit check for debugging");

    match create_connection().await {
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
                    info!("Session created for account_id: {} from IP: {:?}", 
                          account_id, ip_address);
                    Ok(Session {
                        session_id: session_id.clone(),
                        account_id,
                        created_at: now,
                        expires_at,
                        last_accessed: now,
                        ip_address,
                        user_agent,
                        is_active: true,
                    })
                }
                Err(e) => {
                    error!("Failed to create session: {}", e);
                    Err(ServerFnError::new(format!("Failed to create session: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Enhanced session validation with security context checking
#[server]
pub async fn get_session_with_validation(
    session_id: String,
    current_ip: Option<String>,
    current_user_agent: Option<String>,
) -> Result<Option<Session>, ServerFnError> {
    let config = SessionConfig::default();
    
    match create_connection().await {
        Ok(mut conn) => {
            // First, clean up expired sessions if configured
            if config.cleanup_expired {
                let _ = cleanup_expired_sessions().await;
            }

            let result = sqlx::query_as::<_, Session>(
                r#"
                SELECT session_id, account_id, created_at, expires_at, last_accessed, ip_address, user_agent, is_active
                FROM sessions
                WHERE session_id = ?1 AND is_active = 1 AND expires_at > CURRENT_TIMESTAMP
                "#
            )
            .bind(&session_id)
            .fetch_optional(&mut conn)
            .await;

            match result {
                Ok(session_opt) => {
                    if let Some(session) = session_opt {
                        // OWASP: Validate session context
                        let mut is_valid = true;
                        
                        // Check IP address binding
                        if config.bind_to_ip {
                            if let (Some(session_ip), Some(current_ip)) = (&session.ip_address, &current_ip) {
                                if session_ip != current_ip {
                                    warn!("Session IP mismatch for session {}: {} vs {}", 
                                          session_id, session_ip, current_ip);
                                    is_valid = false;
                                }
                            }
                        }
                        
                        // Check User-Agent binding
                        if config.bind_to_user_agent && is_valid {
                            if let (Some(session_ua), Some(current_ua)) = (&session.user_agent, &current_user_agent) {
                                if session_ua != current_ua {
                                    warn!("Session User-Agent mismatch for session {}", session_id);
                                    is_valid = false;
                                }
                            }
                        }
                        
                        // Check idle timeout
                        let idle_duration = Utc::now().signed_duration_since(session.last_accessed);
                        if idle_duration.num_minutes() > config.session_timeout_minutes {
                            warn!("Session {} exceeded idle timeout", session_id);
                            is_valid = false;
                        }
                        
                        if !is_valid {
                            // OWASP: Invalidate suspicious sessions immediately
                            let _ = invalidate_session(session_id.clone()).await;
                            return Ok(None);
                        }
                        
                        // Update last_accessed if configured and session is valid
                        if config.extend_on_access {
                            let _ = update_session_access(session_id.clone()).await;
                        }
                        
                        Ok(Some(session))
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => {
                    error!("Failed to retrieve session: {}", e);
                    Err(ServerFnError::new(format!("Failed to retrieve session: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Validate and retrieve a session (legacy function for compatibility)
#[server]
pub async fn get_session(session_id: String) -> Result<Option<Session>, ServerFnError> {
    // Call the enhanced version without IP/User-Agent validation for backward compatibility
    get_session_with_validation(session_id, None, None).await
}

/// Count active sessions for a user (OWASP: Session management)
#[server]
pub async fn count_active_sessions(account_id: i32) -> Result<u32, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                SELECT COUNT(*) as count
                FROM sessions
                WHERE account_id = ?1 AND is_active = 1 AND expires_at > CURRENT_TIMESTAMP
                "#,
                account_id
            )
            .fetch_one(&mut conn)
            .await;

            match result {
                Ok(row) => {
                    // SQLite returns i64 for COUNT(*), so we need to convert safely
                    let count = row.count.unwrap_or(0) as u32;
                    Ok(count)
                }
                Err(e) => {
                    error!("Failed to count active sessions: {}", e);
                    Err(ServerFnError::new(format!("Failed to count active sessions: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Invalidate the oldest session for a user (OWASP: Session management)
#[server]
pub async fn invalidate_oldest_session(account_id: i32) -> Result<(), ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                UPDATE sessions 
                SET is_active = 0 
                WHERE session_id IN (
                    SELECT session_id FROM sessions 
                    WHERE account_id = ?1 AND is_active = 1 
                    ORDER BY created_at ASC 
                    LIMIT 1
                )
                "#,
                account_id
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(_) => {
                    info!("Invalidated oldest session for account_id: {}", account_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to invalidate oldest session: {}", e);
                    Err(ServerFnError::new(format!("Failed to invalidate oldest session: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Update session last accessed time
#[server]
pub async fn update_session_access(session_id: String) -> Result<(), ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                UPDATE sessions 
                SET last_accessed = CURRENT_TIMESTAMP 
                WHERE session_id = ?1 AND is_active = 1
                "#,
                session_id
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("Failed to update session access: {}", e);
                    Err(ServerFnError::new(format!("Failed to update session access: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Invalidate a session (logout)
#[server]
pub async fn invalidate_session(session_id: String) -> Result<(), ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                UPDATE sessions 
                SET is_active = 0 
                WHERE session_id = ?1
                "#,
                session_id
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(_) => {
                    info!("Session invalidated: {}", session_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to invalidate session: {}", e);
                    Err(ServerFnError::new(format!("Failed to invalidate session: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Invalidate all sessions for a user
#[server]
pub async fn invalidate_all_user_sessions(account_id: i32) -> Result<(), ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                UPDATE sessions 
                SET is_active = 0 
                WHERE account_id = ?1
                "#,
                account_id
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(_) => {
                    info!("All sessions invalidated for account_id: {}", account_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to invalidate user sessions: {}", e);
                    Err(ServerFnError::new(format!("Failed to invalidate user sessions: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Clean up expired sessions
#[server]
pub async fn cleanup_expired_sessions() -> Result<u64, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                r#"
                DELETE FROM sessions 
                WHERE expires_at < CURRENT_TIMESTAMP OR is_active = 0
                "#
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(query_result) => {
                    let deleted_count = query_result.rows_affected();
                    if deleted_count > 0 {
                        info!("Cleaned up {} expired sessions", deleted_count);
                    }
                    Ok(deleted_count)
                }
                Err(e) => {
                    error!("Failed to cleanup expired sessions: {}", e);
                    Err(ServerFnError::new(format!("Failed to cleanup expired sessions: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// Get all active sessions for a user
#[server]
pub async fn get_user_sessions(account_id: i32) -> Result<Vec<Session>, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query_as::<_, Session>(
                r#"
                SELECT session_id, account_id, created_at, expires_at, last_accessed, ip_address, user_agent, is_active
                FROM sessions
                WHERE account_id = ?1 AND is_active = 1 AND expires_at > CURRENT_TIMESTAMP
                ORDER BY last_accessed DESC
                "#
            )
            .bind(account_id)
            .fetch_all(&mut conn)
            .await;

            match result {
                Ok(sessions) => Ok(sessions),
                Err(e) => {
                    error!("Failed to retrieve user sessions: {}", e);
                    Err(ServerFnError::new(format!("Failed to retrieve user sessions: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!("Database connection error: {}", e)))
        }
    }
}

/// OWASP: Session renewal after privilege changes
#[server]
pub async fn renew_session_id(
    old_session_id: String,
    account_id: i32,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<Session, ServerFnError> {
    // Invalidate old session
    invalidate_session(old_session_id).await?;
    
    // Create new session with same user but new ID
    create_session(account_id, ip_address, user_agent).await
} 