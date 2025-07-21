/// Database connection and migration management using SQLx
///
/// This module provides utilities for connecting to the SQLite database
/// and running migrations. It replaces the previous Diesel-based approach
/// with SQLx for better async support and modern Rust practices.

#[cfg(feature = "server")]
use sqlx::{sqlite::SqlitePool, SqliteConnection, Connection, Row,migrate::MigrateError};
#[cfg(feature = "server")]
use std::env;

/// Database connection pool type alias for cleaner code
#[cfg(feature = "server")]
pub type DbPool = SqlitePool;

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
/// # Returns
///
/// A `Result` containing either a `SqliteConnection` or a `sqlx::Error`
#[cfg(feature = "server")]
pub async fn create_connection() -> Result<SqliteConnection, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());

    SqliteConnection::connect(&database_url).await
}
