//! Database connection pool management.
//!
//! Provides functions for creating and managing PostgreSQL connection pools.

use sqlx::postgres::{PgPool, PgPoolOptions};

/// Type alias for PostgreSQL connection pool.
pub type DbPool = PgPool;

/// Create a new PostgreSQL connection pool.
///
/// Initializes a connection pool with a maximum of 5 connections.
///
/// # Arguments
///
/// * `database_url` - PostgreSQL connection string
///
/// # Returns
///
/// A connection pool ready to use for database operations
///
/// # Errors
///
/// Returns an error if the database connection cannot be established
///
/// # Example
///
/// ```no_run
/// # use taskur_backend::db::create_pool;
/// # async fn example() {
/// let pool = create_pool("postgresql://user:pass@localhost/db")
///     .await
///     .expect("Failed to create pool");
/// # }
/// ```
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
