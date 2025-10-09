use crate::{
    config::Config,
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse},
    utils,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use utoipa;
use uuid::Uuid;
use validator::Validate;

/// Register a new user account.
///
/// Creates a new user with the provided email and password.
/// Passwords are securely hashed using bcrypt before storage.
/// Returns a JWT token and user information upon successful registration.
///
/// # Errors
///
/// Returns an error if:
/// - Email is already registered
/// - Validation fails (invalid email format or password too short)
/// - Database operation fails
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid request or email already registered"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<CreateUserRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(pool.as_ref())
    .await?;

    if existing_user.is_some() {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = utils::hash_password(&body.password)?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&body.email)
    .bind(&password_hash)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    // Generate JWT token
    let config = Config::from_env();
    let token = utils::create_token(user.id, &config.jwt_secret)?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Authenticate a user and generate JWT token.
///
/// Validates user credentials and returns a JWT token for authenticated requests.
/// The token should be included in the Authorization header as "Bearer {token}".
///
/// # Errors
///
/// Returns an error if:
/// - Credentials are invalid (wrong email or password)
/// - Validation fails
/// - Database operation fails
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    // Verify password
    let valid = utils::verify_password(&body.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let config = Config::from_env();
    let token = utils::create_token(user.id, &config.jwt_secret)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Get current authenticated user information.
///
/// Retrieves the profile information of the currently authenticated user.
/// Requires a valid JWT token in the Authorization header.
///
/// # Errors
///
/// Returns an error if:
/// - User is not authenticated
/// - User not found in database
/// - Database operation fails
#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Current user information", body = UserResponse),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}
