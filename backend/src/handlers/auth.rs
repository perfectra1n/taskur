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
use uuid::Uuid;
use validator::Validate;

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
