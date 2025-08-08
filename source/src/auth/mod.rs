//! # Authentication and Session Management
//! 
//! This module provides OWASP-compliant session management with secure cookie handling
//! for Dioxus fullstack applications.
//! 
//! ## Architecture
//! 
//! The session management system consists of several components:
//! 
//! - **Session Management** (`session.rs`): Server-side session storage in SQLite with
//!   enhanced security features including IP binding, User-Agent binding, session timeouts,
//!   and concurrent session limits.
//! 
//! - **Cookie Management** (`cookies.rs`): Client-side cookie handling with OWASP security
//!   attributes including Secure, HttpOnly (server-side), SameSite, and proper expiration.
//! 
//! - **Authentication** (`authentication.rs`): Login/logout functionality with Argon2
//!   password hashing and session integration.
//! 
//! ## Security Features
//! 
//! Following OWASP Session Management guidelines:
//! 
//! - **Secure Session IDs**: Generated using UUID v4 for cryptographic randomness
//! - **Session Binding**: Sessions are bound to IP addresses and User-Agent strings
//! - **Automatic Timeouts**: Configurable idle timeouts and absolute session expiration
//! - **Concurrent Session Limits**: Prevents session fixation by limiting active sessions
//! - **Secure Cookies**: Uses Secure, HttpOnly, and SameSite attributes
//! - **Password Security**: Argon2 hashing with random salts
//! - **Session Cleanup**: Automatic cleanup of expired sessions
//! 
//! ## Usage Example
//! 
//! ```rust
//! use crate::auth::{SessionCookieManager, login_with_session_and_cookies, CurrentUser};
//! 
//! // Login and store session in cookie
//! let response = login_with_session_and_cookies(
//!     username,
//!     password,
//!     Some("127.0.0.1".to_string()),
//!     Some("User-Agent".to_string())
//! ).await?;
//! 
//! if response.success {
//!     if let Some(session_id) = response.session_id {
//!         // Store in client-side cookie
//!         SessionCookieManager::store_session_sync(session_id)?;
//!     }
//! }
//! 
//! // Retrieve session from cookie
//! if let Some(session_id) = SessionCookieManager::get_session_id_sync() {
//!     let current_user = get_current_user(session_id).await?;
//! }
//! 
//! // Logout and clear cookies
//! logout_with_cookies(session_id).await?;
//! SessionCookieManager::clear_session_sync()?;
//! ```
//! 
//! ## Development vs Production
//! 
//! The cookie configuration is set for development by default:
//! - `Secure: false` (allows HTTP in development)
//! - `SameSite: Lax` (more permissive for development)
//! 
//! For production, update `CookieConfig::default()` to:
//! - `Secure: true` (requires HTTPS)
//! - `SameSite: Strict` (maximum CSRF protection)

pub mod authentication;
pub mod session;
pub mod cookies;

pub use authentication::*;
pub use session::*;
pub use cookies::*;

