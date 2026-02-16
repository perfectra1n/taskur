use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// User database model representing a registered user in the system.
///
/// This struct contains all user information stored in the database,
/// including authentication credentials and timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's email address (used for authentication)
    pub email: String,
    /// Bcrypt hashed password (never sent in API responses)
    #[serde(skip_serializing)]
    #[schema(value_type = String, example = "$2b$12$...")]
    pub password_hash: String,
    /// Timestamp when the user account was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user account was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request payload for user registration.
///
/// Contains the required fields for creating a new user account.
/// Email must be valid and password must meet minimum length requirements.
#[derive(Debug, Deserialize, validator::Validate, ToSchema)]
pub struct CreateUserRequest {
    /// User's email address (must be valid email format)
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com")]
    pub email: String,
    /// User's password (minimum 8 characters)
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(min_length = 8, example = "securePassword123")]
    pub password: String,
}

/// Request payload for user login.
///
/// Contains credentials required to authenticate a user.
#[derive(Debug, Deserialize, validator::Validate, ToSchema, TS)]
#[ts(export)]
pub struct LoginRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com")]
    pub email: String,
    /// User's password
    #[schema(example = "securePassword123")]
    pub password: String,
}

/// Response payload for successful authentication.
///
/// Contains a JWT token and user information after successful
/// registration or login.
#[derive(Debug, Serialize, ToSchema, TS)]
#[ts(export)]
pub struct AuthResponse {
    /// JWT authentication token (use in Authorization header)
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    /// User information
    pub user: UserResponse,
}

/// Public user information response.
///
/// Contains safe-to-expose user data without sensitive information
/// like password hashes.
#[derive(Debug, Serialize, ToSchema, TS)]
#[ts(export)]
pub struct UserResponse {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's email address
    #[schema(example = "user@example.com")]
    pub email: String,
    /// Timestamp when the user account was created
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
