use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateTagRequest, Tag},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

/// List all tags for the authenticated user.
#[utoipa::path(
    get,
    path = "/api/tags",
    responses(
        (status = 200, description = "List of tags", body = Vec<Tag>),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tags",
    security(("bearer_auth" = []))
)]
pub async fn list_tags(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT * FROM tags WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(auth.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(tags))
}

/// Create a new tag.
#[utoipa::path(
    post,
    path = "/api/tags",
    request_body = CreateTagRequest,
    responses(
        (status = 201, description = "Tag created successfully", body = Tag),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tags",
    security(("bearer_auth" = []))
)]
pub async fn create_tag(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    body: web::Json<CreateTagRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (id, user_id, name, color, created_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(&body.name)
    .bind(&body.color)
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(tag))
}

/// Delete a tag.
#[utoipa::path(
    delete,
    path = "/api/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag ID")
    ),
    responses(
        (status = 204, description = "Tag deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Tag not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tags",
    security(("bearer_auth" = []))
)]
pub async fn delete_tag(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let tag_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM tags WHERE id = $1 AND user_id = $2"
    )
    .bind(tag_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tag not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}
