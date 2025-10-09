//! Application configuration management.
//!
//! Loads configuration from environment variables.

use std::env;

/// Application configuration.
///
/// Contains all configuration values loaded from environment variables.
#[derive(Clone)]
pub struct Config {
    /// PostgreSQL database connection URL
    pub database_url: String,
    /// Secret key for JWT token signing
    pub jwt_secret: String,
    /// Server host address
    pub host: String,
    /// Server port number
    pub port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// Reads configuration from environment variables with sensible defaults
    /// where applicable.
    ///
    /// # Required Environment Variables
    ///
    /// - `DATABASE_URL`: PostgreSQL connection string
    /// - `JWT_SECRET`: Secret key for JWT signing
    ///
    /// # Optional Environment Variables
    ///
    /// - `HOST`: Server host (default: "127.0.0.1")
    /// - `PORT`: Server port (default: "8080")
    ///
    /// # Panics
    ///
    /// Panics if required environment variables are not set or if PORT
    /// cannot be parsed as a number.
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            host: env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
        }
    }
}
