//! JWT token utilities for authentication.
//!
//! Provides functions for creating and verifying JWT tokens used for
//! authenticating API requests.

use crate::errors::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT token claims.
///
/// Contains the information embedded in a JWT token including
/// user identification and expiration information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID as string)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
}

impl Claims {
    /// Create new JWT claims for a user.
    ///
    /// Tokens are valid for 7 days from creation.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user to create claims for
    ///
    /// # Returns
    ///
    /// A `Claims` struct with the user ID and appropriate timestamps
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::days(7)).timestamp();

        Self {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp(),
        }
    }

    /// Extract the user ID from the claims.
    ///
    /// # Returns
    ///
    /// The user's UUID if the subject can be parsed, otherwise an error
    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
}

/// Create a JWT token for a user.
///
/// Generates a signed JWT token that can be used for authentication.
///
/// # Arguments
///
/// * `user_id` - UUID of the user to create a token for
/// * `secret` - Secret key used to sign the token
///
/// # Returns
///
/// A signed JWT token string
///
/// # Errors
///
/// Returns an error if token encoding fails
pub fn create_token(user_id: Uuid, secret: &str) -> AppResult<String> {
    let claims = Claims::new(user_id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

/// Verify and decode a JWT token.
///
/// Validates the token signature and expiration, then returns the claims.
///
/// # Arguments
///
/// * `token` - JWT token string to verify
/// * `secret` - Secret key used to verify the token signature
///
/// # Returns
///
/// The decoded claims if the token is valid
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is invalid
pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
