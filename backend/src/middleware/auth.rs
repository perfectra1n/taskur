//! JWT authentication middleware.
//!
//! Provides authentication extraction for protected routes.

use crate::{config::Config, errors::AppError, utils};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

/// Authenticated user extractor.
///
/// This struct can be used as a parameter in handler functions to automatically
/// verify JWT authentication and extract the user ID from the token.
///
/// # Example
///
/// ```no_run
/// use actix_web::{web, HttpResponse};
/// use taskur_backend::middleware::AuthenticatedUser;
///
/// async fn protected_route(auth: AuthenticatedUser) -> HttpResponse {
///     HttpResponse::Ok().json(format!("User ID: {}", auth.user_id))
/// }
/// ```
pub struct AuthenticatedUser {
    /// ID of the authenticated user extracted from the JWT token
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    /// Extract authenticated user from request.
    ///
    /// Validates the JWT token in the Authorization header and extracts
    /// the user ID. Returns an error if authentication fails.
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = Config::from_env();

        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => return ready(Err(AppError::AuthenticationError(
                "Missing authorization header".to_string()
            ))),
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => return ready(Err(AppError::AuthenticationError(
                "Invalid authorization header".to_string()
            ))),
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(AppError::AuthenticationError(
                "Invalid authorization format".to_string()
            )));
        }

        let token = &auth_str[7..];

        match utils::verify_token(token, &config.jwt_secret) {
            Ok(claims) => match claims.user_id() {
                Ok(user_id) => ready(Ok(AuthenticatedUser { user_id })),
                Err(_) => ready(Err(AppError::AuthenticationError(
                    "Invalid user ID in token".to_string()
                ))),
            },
            Err(e) => ready(Err(e)),
        }
    }
}
