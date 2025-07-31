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

/// Session configuration
pub struct SessionConfig {
    pub duration_hours: i64,
    pub extend_on_access: bool,
    pub cleanup_expired: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            duration_hours: 24, // 24 hours default
            extend_on_access: true,
            cleanup_expired: true,
        }
    }
}

/// Create a new session for a user
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
                    info!("Session created for account_id: {}", account_id);
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

/// Validate and retrieve a session
#[server]
pub async fn get_session(session_id: String) -> Result<Option<Session>, ServerFnError> {
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
                        // Update last_accessed if configured
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