use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{Comment, CreateCommentRequest, Task, UpdateCommentRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

/// List all comments for a task.
///
/// Returns comments ordered by creation date.
#[utoipa::path(
    get,
    path = "/api/tasks/{task_id}/comments",
    params(
        ("task_id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "List of comments", body = Vec<Comment>),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Comments",
    security(("bearer_auth" = []))
)]
pub async fn list_comments(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    // Verify task ownership
    let _task = sqlx::query_as::<_, Task>(
        "SELECT *, NULL::real AS relevance FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE task_id = $1 ORDER BY created_at ASC"
    )
    .bind(task_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(comments))
}

/// Create a new comment on a task.
///
/// Adds a comment to the specified task.
#[utoipa::path(
    post,
    path = "/api/tasks/{task_id}/comments",
    request_body = CreateCommentRequest,
    params(
        ("task_id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 201, description = "Comment created successfully", body = Comment),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Comments",
    security(("bearer_auth" = []))
)]
pub async fn create_comment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<CreateCommentRequest>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Verify task ownership
    let _task = sqlx::query_as::<_, Task>(
        "SELECT *, NULL::real AS relevance FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let comment = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (id, task_id, user_id, content, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(task_id)
    .bind(auth.user_id)
    .bind(&body.content)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(comment))
}

/// Update an existing comment.
///
/// Updates the content of a comment.
#[utoipa::path(
    put,
    path = "/api/tasks/{task_id}/comments/{comment_id}",
    request_body = UpdateCommentRequest,
    params(
        ("task_id" = Uuid, Path, description = "Task ID"),
        ("comment_id" = Uuid, Path, description = "Comment ID")
    ),
    responses(
        (status = 200, description = "Comment updated successfully", body = Comment),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Comment or task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Comments",
    security(("bearer_auth" = []))
)]
pub async fn update_comment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateCommentRequest>,
) -> AppResult<HttpResponse> {
    let (task_id, comment_id) = path.into_inner();

    // Verify task ownership
    let _task = sqlx::query_as::<_, Task>(
        "SELECT *, NULL::real AS relevance FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let updated_comment = sqlx::query_as::<_, Comment>(
        "UPDATE comments
         SET content = $1, updated_at = $2
         WHERE id = $3 AND task_id = $4 AND user_id = $5
         RETURNING *"
    )
    .bind(&body.content)
    .bind(Utc::now())
    .bind(comment_id)
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    Ok(HttpResponse::Ok().json(updated_comment))
}

/// Delete a comment.
///
/// Permanently deletes a comment.
#[utoipa::path(
    delete,
    path = "/api/tasks/{task_id}/comments/{comment_id}",
    params(
        ("task_id" = Uuid, Path, description = "Task ID"),
        ("comment_id" = Uuid, Path, description = "Comment ID")
    ),
    responses(
        (status = 204, description = "Comment deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Comments",
    security(("bearer_auth" = []))
)]
pub async fn delete_comment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> AppResult<HttpResponse> {
    let (task_id, comment_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM comments WHERE id = $1 AND task_id = $2 AND user_id = $3"
    )
    .bind(comment_id)
    .bind(task_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Comment not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}
