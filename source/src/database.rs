/// Database connection and migration management using SQLx
///
/// This module provides utilities for connecting to the SQLite database
/// and running migrations. It replaces the previous Diesel-based approach
/// with SQLx for better async support and modern Rust practices.

#[cfg(feature = "server")]
use sqlx::{sqlite::SqlitePool, SqliteConnection, Connection, Row,migrate::MigrateError};
#[cfg(feature = "server")]
use std::env;
#[cfg(feature = "server")]
use dioxus::logger::tracing::{info, error, warn};

/// Database connection pool type alias for cleaner code
#[cfg(feature = "server")]
pub type DbPool = SqlitePool;

/// Creates a direct database connection without initialization
/// This is used internally to avoid circular dependencies during initialization
#[cfg(feature = "server")]
async fn create_direct_connection() -> Result<SqliteConnection, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());

    SqliteConnection::connect(&database_url).await
}

/// Initialize database and run migrations
/// This should be called once at application startup
#[cfg(feature = "server")]
pub async fn initialize_database() -> Result<(), sqlx::Error> {
    info!("Initializing database...");
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());
    
    info!("Database URL: {}", database_url);
    
    // Create connection pool
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => {
            info!("Database connection pool created successfully");
            pool
        }
        Err(e) => {
            error!("Failed to create database connection pool: {}", e);
            return Err(e);
        }
    };
    
    // Run migrations
    info!("Running database migrations...");
    match run_migrations(&pool).await {
        Ok(_) => {
            info!("Database migrations completed successfully");
        }
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            // Convert MigrateError to sqlx::Error for consistency
            return Err(sqlx::Error::Migrate(Box::new(e)));
        }
    }
    
    // Close the pool
    pool.close().await;
    info!("Database initialization completed");
    
    // Create a test admin account if it doesn't exist
    // Use direct connection to avoid circular dependency
    match create_test_admin_account_direct().await {
        Ok(_) => info!("Test admin account verified/created"),
        Err(e) => warn!("Failed to create test admin account: {}", e),
    }
    
    Ok(())
}

/// Create a test admin account for development/testing using direct connection
#[cfg(feature = "server")]
async fn create_test_admin_account_direct() -> Result<(), sqlx::Error> {
    use argon2::{Argon2, PasswordHasher};
    use password_hash::{rand_core::OsRng, SaltString};
    
    // Use direct connection to avoid circular dependency
    let mut conn = create_direct_connection().await?;
    
    // Check if admin account already exists
    let existing_admin = sqlx::query!(
        "SELECT account_id FROM accounts WHERE username = ?1",
        "admin"
    )
    .fetch_optional(&mut conn)
    .await?;
    
    if existing_admin.is_some() {
        info!("Admin account already exists");
        return Ok(());
    }
    
    info!("Creating test admin account...");
    
    // Hash the password directly here to avoid circular dependencies
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password("admin123".as_bytes(), &salt)
        .map_err(|e| {
            error!("Failed to hash password: {}", e);
            sqlx::Error::Protocol("Failed to hash password".to_string())
        })?;
    
    let password_hash_str = password_hash.to_string();
    
    // Create admin account directly
    let result = sqlx::query!(
        "INSERT INTO accounts (username, password_hash, role_id) VALUES (?1, ?2, ?3)",
        "admin",
        password_hash_str,
        1 // Admin role ID
    )
    .execute(&mut conn)
    .await;
    
    match result {
        Ok(_) => {
            info!("Test admin account created successfully (username: admin, password: admin123)");
            Ok(())
        }
        Err(e) => {
            error!("Failed to create test admin account: {}", e);
            Err(e)
        }
    }
}

/// Creates a new database connection pool
///
/// This function reads the DATABASE_URL environment variable and creates
/// a connection pool that can be shared across the application.
///
/// # Returns
///
/// A `Result` containing either a `SqlitePool` or a `sqlx::Error`
///
/// # Examples
///
/// ```rust
/// let pool = create_pool().await?;
/// ```
#[cfg(feature = "server")]
pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());

    SqlitePool::connect(&database_url).await
}

/// Runs database migrations
///
/// This function applies all pending migrations to the database.
/// It should be called during application startup.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool
///
/// # Returns
///
/// A `Result` indicating success or failure of the migration process
///
/// # Examples
///
/// ```rust
/// let pool = create_pool().await?;
/// run_migrations(&pool).await?;
/// ```
#[cfg(feature = "server")]
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

/// Creates a single database connection
///
/// This is a convenience function for cases where you need a single connection
/// instead of a pool. Generally, using the pool is recommended for better
/// performance and connection management.
/// 
/// This function will automatically initialize the database and run migrations
/// if the database doesn't exist or is not properly set up.
///
/// # Returns
///
/// A `Result` containing either a `SqliteConnection` or a `sqlx::Error`
#[cfg(feature = "server")]
pub async fn create_connection() -> Result<SqliteConnection, sqlx::Error> {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());

    // Try to connect first
    match create_direct_connection().await {
        Ok(conn) => {
            info!("Database connection established successfully");
            Ok(conn)
        }
        Err(e) => {
            // If connection fails and we haven't initialized yet, try to initialize
            if !INITIALIZED.load(Ordering::Relaxed) {
                warn!("Initial database connection failed: {}, attempting to initialize...", e);
                
                match initialize_database().await {
                    Ok(_) => {
                        info!("Database initialized, retrying connection...");
                        INITIALIZED.store(true, Ordering::Relaxed);
                        // Try to connect again after initialization
                        create_direct_connection().await
                    }
                    Err(init_error) => {
                        error!("Failed to initialize database: {}", init_error);
                        // Return the original connection error
                        Err(e)
                    }
                }
            } else {
                // Already tried initialization, return the connection error
                Err(e)
            }
        }
    }
}
