use dioxus::prelude::*;
use dioxus::logger::tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};

// Cookie configuration following OWASP recommendations
pub struct CookieConfig {
    pub name: String,
    pub max_age_seconds: i64,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub path: String,
    pub domain: Option<String>,
}

#[derive(Clone, Debug)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Default for CookieConfig {
    fn default() -> Self {
        Self {
            name: "session_id".to_string(),
            max_age_seconds: 24 * 60 * 60, // 24 hours
            // Set to false for development (localhost), should be true in production
            secure: false,  
            http_only: true, // OWASP: Prevent XSS attacks
            same_site: SameSite::Lax, // OWASP: Prevent CSRF attacks (Lax for development)
            path: "/".to_string(),
            domain: None, // Let browser determine domain
        }
    }
}

/// High-level session cookie management
/// This provides a unified interface that works on both server and client
pub struct SessionCookieManager;

impl SessionCookieManager {
    /// Store session ID in a secure cookie
    pub async fn store_session(session_id: String) -> Result<(), ServerFnError> {
        let config = CookieConfig::default();
        
        #[cfg(feature = "web")]
        {
            // Client-side: Set cookie using document.cookie
            if let Err(e) = set_client_cookie(&config.name, &session_id, Some(config.max_age_seconds)) {
                error!("Failed to set client cookie: {:?}", e);
                return Err(ServerFnError::new("Failed to store session cookie"));
            }
            info!("Session stored in client-side cookie");
        }
        
        #[cfg(not(feature = "web"))]
        {
            info!("Session storage requested: {}", session_id);
            // For server-side rendering, we'll log the session but actual cookie setting
            // would need HTTP response integration
            warn!("Server-side cookie setting requires HTTP response integration");
        }
        
        Ok(())
    }
    
    /// Retrieve session ID from cookie
    pub async fn get_session() -> Result<Option<String>, ServerFnError> {
        let config = CookieConfig::default();
        
        #[cfg(feature = "web")]
        {
            // Client-side: Read from document.cookie
            match get_client_cookie(&config.name) {
                Ok(session_id) => {
                    if let Some(ref id) = session_id {
                        info!("Session ID retrieved from client cookie: {}", id);
                    } else {
                        info!("No session cookie found on client");
                    }
                    Ok(session_id)
                }
                Err(e) => {
                    error!("Failed to get client cookie: {:?}", e);
                    Ok(None)
                }
            }
        }
        
        #[cfg(not(feature = "web"))]
        {
            info!("Session retrieval requested from server context");
            // For server-side, we would need to access HTTP request headers
            // This is a placeholder that always returns None for now
            warn!("Server-side cookie retrieval requires HTTP request integration");
            Ok(None)
        }
    }
    
    /// Clear session cookie
    pub async fn clear_session() -> Result<(), ServerFnError> {
        let config = CookieConfig::default();
        
        #[cfg(feature = "web")]
        {
            if let Err(e) = clear_client_cookie(&config.name) {
                error!("Failed to clear client cookie: {:?}", e);
                return Err(ServerFnError::new("Failed to clear session cookie"));
            }
            info!("Session cookie cleared on client");
        }
        
        #[cfg(not(feature = "web"))]
        {
            info!("Session clearing requested");
            warn!("Server-side cookie clearing requires HTTP response integration");
        }
        
        Ok(())
    }

    /// Get session ID from client-side cookies (sync version for use in components)
    pub fn get_session_id_sync() -> Option<String> {
        #[cfg(feature = "web")]
        {
            let config = CookieConfig::default();
            match get_client_cookie(&config.name) {
                Ok(session_id) => session_id,
                Err(_) => None,
            }
        }
        
        #[cfg(not(feature = "web"))]
        {
            None
        }
    }

    /// Store session ID in client-side cookies (sync version for use in components)
    pub fn store_session_sync(session_id: String) -> Result<(), String> {
        #[cfg(feature = "web")]
        {
            let config = CookieConfig::default();
            match set_client_cookie(&config.name, &session_id, Some(config.max_age_seconds)) {
                Ok(_) => {
                    info!("Session cookie stored successfully");
                    Ok(())
                }
                Err(e) => Err(format!("Failed to store session: {:?}", e)),
            }
        }
        
        #[cfg(not(feature = "web"))]
        {
            info!("Session storage requested for: {}", session_id);
            Ok(())
        }
    }

    /// Clear session ID from client-side cookies (sync version for use in components)
    pub fn clear_session_sync() -> Result<(), String> {
        #[cfg(feature = "web")]
        {
            let config = CookieConfig::default();
            match clear_client_cookie(&config.name) {
                Ok(_) => {
                    info!("Session cookie cleared successfully");
                    Ok(())
                }
                Err(e) => Err(format!("Failed to clear session: {:?}", e)),
            }
        }
        
        #[cfg(not(feature = "web"))]
        {
            info!("Session clearing requested");
            Ok(())
        }
    }
}

/// Client-side cookie management for browser
#[cfg(feature = "web")]
mod client_cookies {
    use super::*;
    use web_sys::window;
    use wasm_bindgen::JsValue;

    /// Set cookie on client-side (browser)
    /// Note: Client-side cookies cannot set HttpOnly flag (security limitation)
    pub fn set_client_cookie(name: &str, value: &str, max_age_seconds: Option<i64>) -> Result<(), JsValue> {
        let window = window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        
        let config = CookieConfig::default();
        let max_age = max_age_seconds.unwrap_or(config.max_age_seconds);
        
        // Build cookie string with security attributes
        // Note: HttpOnly cannot be set from client-side JavaScript (this is by design for security)
        let mut cookie_str = format!("{}={}; Path={}; Max-Age={}", 
            name, value, config.path, max_age);
            
        // Add Secure flag if configured (should be true for production)
        if config.secure {
            cookie_str.push_str("; Secure");
        }
        
        // Add SameSite attribute
        match config.same_site {
            SameSite::Strict => cookie_str.push_str("; SameSite=Strict"),
            SameSite::Lax => cookie_str.push_str("; SameSite=Lax"),
            SameSite::None => cookie_str.push_str("; SameSite=None"),
        }
        
        // Add domain if specified
        if let Some(domain) = &config.domain {
            cookie_str.push_str(&format!("; Domain={}", domain));
        }
        
        // Use the correct method for setting cookies
        use wasm_bindgen::JsCast;
        let html_document = document.dyn_into::<web_sys::HtmlDocument>()?;
        html_document.set_cookie(&cookie_str)?;
        info!("Set client cookie: {} = {} (expires in {} seconds)", name, value, max_age);
        
        Ok(())
    }
    
    /// Get cookie value from client-side
    pub fn get_client_cookie(name: &str) -> Result<Option<String>, JsValue> {
        let window = window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        
        // Use the correct method for getting cookies
        use wasm_bindgen::JsCast;
        let html_document = document.dyn_into::<web_sys::HtmlDocument>()?;
        let cookie_string = html_document.cookie()?;
        
        debug!("All cookies: {}", cookie_string);
        
        // Parse cookies to find the one we want
        for cookie in cookie_string.split(';') {
            let cookie = cookie.trim();
            if let Some(eq_pos) = cookie.find('=') {
                let (key, value) = cookie.split_at(eq_pos);
                if key.trim() == name {
                    let value = &value[1..]; // Remove the '=' character
                    debug!("Found cookie {} = {}", name, value);
                    return Ok(Some(value.to_string()));
                }
            }
        }
        
        debug!("Cookie {} not found", name);
        Ok(None)
    }
    
    /// Clear cookie on client-side
    pub fn clear_client_cookie(name: &str) -> Result<(), JsValue> {
        let window = window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        
        let config = CookieConfig::default();
        
        // Set cookie to expire immediately
        let cookie_str = format!("{}=; Path={}; Max-Age=-1; Expires=Thu, 01 Jan 1970 00:00:00 GMT", 
            name, config.path);
            
        // Use the correct method for setting cookies
        use wasm_bindgen::JsCast;
        let html_document = document.dyn_into::<web_sys::HtmlDocument>()?;
        html_document.set_cookie(&cookie_str)?;
        info!("Cleared client cookie: {}", name);
        
        Ok(())
    }
}

#[cfg(feature = "web")]
pub use client_cookies::*;

/// OWASP-compliant session validation with additional security checks
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SessionSecurityContext {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SessionSecurityContext {
    /// Validate that the session context matches current request
    /// This implements OWASP recommendation for binding sessions to client properties
    pub fn validate_context(
        &self,
        current_ip: Option<&str>,
        current_user_agent: Option<&str>,
    ) -> bool {
        // Check IP address binding (if available)
        if let (Some(session_ip), Some(current_ip)) = (&self.ip_address, current_ip) {
            if session_ip != current_ip {
                warn!("Session IP mismatch: {} vs {}", session_ip, current_ip);
                return false;
            }
        }
        
        // Check User-Agent binding (if available)
        if let (Some(session_ua), Some(current_ua)) = (&self.user_agent, current_user_agent) {
            if session_ua != current_ua {
                warn!("Session User-Agent mismatch");
                return false;
            }
        }
        
        true
    }
    
    /// Check if session is within acceptable age limits
    pub fn is_within_age_limit(&self, max_age_hours: i64) -> bool {
        let now = chrono::Utc::now();
        let age = now.signed_duration_since(self.created_at);
        age.num_hours() <= max_age_hours
    }
} 