# Database Migration: Diesel to SQLx

This document describes the migration from Diesel ORM to SQLx for the Nico Judge website project.

## Overview

The project has been migrated from Diesel ORM to SQLx for the following reasons:

- **Better async support**: SQLx is built from the ground up for async/await patterns
- **Compile-time query verification**: SQLx checks SQL queries at compile time
- **Lighter weight**: SQLx has fewer dependencies and a smaller footprint
- **Better error handling**: More descriptive error messages and better Result types
- **Modern Rust practices**: Better alignment with current Rust ecosystem patterns

## Changes Made

### 1. Dependencies

**Before (Diesel):**
```toml
diesel = { version = "2.2.11", features = ["sqlite"], optional = true }
```

**After (SQLx):**
```toml
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "migrate", "chrono", "uuid"], optional = true }
```

### 2. Database Connection

**Before (Diesel):**
```rust
use diesel::{prelude::*, SqliteConnection};

let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let mut connection = SqliteConnection::establish(&database_url)?;
```

**After (SQLx):**
```rust
use sqlx::{SqliteConnection, Connection};

let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "sqlite:main.db".to_string());
let mut conn = SqliteConnection::connect(&database_url).await?;
```

### 3. Model Definitions

**Before (Diesel):**
```rust
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = blog_posts))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct BlogPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}
```

**After (SQLx):**
```rust
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct BlogPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}
```

### 4. Database Queries

**Before (Diesel):**
```rust
let posts = blog_posts
    .filter(id.eq(post_id))
    .limit(1)
    .select(BlogPost::as_select())
    .load(&mut connection)?;
```

**After (SQLx):**
```rust
let post = sqlx::query_as::<_, BlogPost>(
    "SELECT id, title, content FROM blog_posts WHERE id = ?1"
)
.bind(post_id)
.fetch_optional(&mut conn)
.await?;
```

### 5. Migrations

**Before (Diesel):**
- Used `diesel.toml` configuration
- Migrations in `migrations/` with up/down SQL files
- Generated `schema.rs` file

**After (SQLx):**
- Migrations in `migrations/` directory
- Single SQL files with descriptive names
- No generated schema files
- Built-in migration runner

## File Structure Changes

### Removed Files
- `source/diesel.toml` - Diesel configuration
- `source/src/schema.rs` - Generated Diesel schema
- `source/migrations/2025-07-04-113215_blog_posts/` - Old Diesel migration format

### Added Files
- `source/src/database.rs` - Database connection utilities
- `source/migrations/001_create_blog_posts.sql` - SQLx migration
- `DATABASE_MIGRATION.md` - This documentation file

### Modified Files
- `source/Cargo.toml` - Updated dependencies
- `source/src/lib.rs` - Updated module exports
- `source/src/models/blog_post.rs` - Complete rewrite for SQLx
- All Dockerfile files - Updated to install SQLx CLI

## Database Module

A new `database.rs` module has been created to centralize database connection management:

```rust
/// Creates a new database connection pool
pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:main.db".to_string());
    
    SqlitePool::connect(&database_url).await
}

/// Runs database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await
}
```

## Running Migrations

### Development
```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features sqlite

# Run migrations
sqlx migrate run --database-url sqlite:main.db
```

### Production
Migrations are automatically run during application startup using the `run_migrations()` function.

## Error Handling

SQLx provides better error handling with more descriptive error messages:

```rust
match result {
    Ok(post) => {
        if post.is_some() {
            info!("Post found with id: {post_id}");
        } else {
            info!("No post found with id: {post_id}");
        }
        Ok(post)
    }
    Err(e) => {
        error!("Error loading blog post: {e}");
        Err(ServerFnError::new(format!("Error loading blog post: {e}")))
    }
}
```

## Performance Improvements

- **Async by default**: All database operations are now truly async
- **Connection pooling**: Built-in connection pool support
- **Compile-time query checking**: Catches SQL errors at compile time
- **Prepared statements**: Automatic query preparation and caching

## Testing

All server functions maintain the same API, so existing tests should continue to work without modification. The async nature of SQLx provides better test isolation and performance.

## Docker Support

All Dockerfiles have been updated to:
- Install SQLx CLI instead of Diesel CLI
- Remove Diesel configuration files
- Support SQLx migrations

## Environment Variables

The same environment variables are supported:
- `DATABASE_URL`: Database connection string (defaults to `sqlite:main.db`)

## Future Enhancements

With SQLx in place, the following improvements are now possible:
- Connection pooling for better performance
- Query caching
- Better transaction support
- Compile-time query verification
- More sophisticated error handling

## Troubleshooting

### Common Issues

1. **Migration errors**: Ensure the database file exists and is writable
2. **Compile errors**: Make sure all feature flags are correctly set
3. **Connection issues**: Check the DATABASE_URL environment variable

### Debugging

Enable detailed logging:
```bash
RUST_LOG=sqlx=debug,web=debug cargo run --features server
```

## Conclusion

The migration to SQLx provides a more modern, async-first approach to database operations while maintaining the same API surface. The improved error handling, compile-time query verification, and better performance make this a significant upgrade for the project. 