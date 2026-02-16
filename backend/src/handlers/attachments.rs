use crate::{
    config::Config,
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{Attachment, AttachmentResponse},
    utils,
};
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt;
use std::io::Write;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const UPLOAD_DIR: &str = "./uploads";

/// Upload file attachments to a task.
///
/// Accepts multipart/form-data file uploads. Maximum file size is 10MB.
#[utoipa::path(
    post,
    path = "/api/tasks/{task_id}/attachments",
    params(
        ("task_id" = Uuid, Path, description = "Task ID")
    ),
    request_body(content_type = "multipart/form-data", description = "File upload"),
    responses(
        (status = 201, description = "Files uploaded successfully", body = Vec<AttachmentResponse>),
        (status = 400, description = "Invalid request or file too large"),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Attachments",
    security(("bearer_auth" = []))
)]
pub async fn upload_attachment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    mut payload: Multipart,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    // Create upload directory if it doesn't exist
    std::fs::create_dir_all(UPLOAD_DIR)
        .map_err(|e| AppError::InternalError(format!("Failed to create upload directory: {}", e)))?;

    let mut attachments = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| AppError::InternalError(e.to_string()))?;

        let original_filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            .ok_or_else(|| AppError::ValidationError("Filename is required".to_string()))?;

        let mime_type = field.content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let filename = format!("{}-{}", file_id, original_filename);
        let file_path = format!("{}/{}", UPLOAD_DIR, filename);

        // Save file
        let mut file = std::fs::File::create(&file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to create file: {}", e)))?;

        let mut file_size: i64 = 0;

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| AppError::InternalError(e.to_string()))?;

            file_size += data.len() as i64;

            if file_size > MAX_FILE_SIZE as i64 {
                // Delete the partially written file
                let _ = std::fs::remove_file(&file_path);
                return Err(AppError::ValidationError(
                    "File size exceeds maximum limit of 10MB".to_string(),
                ));
            }

            file.write_all(&data)
                .map_err(|e| AppError::InternalError(format!("Failed to write file: {}", e)))?;
        }

        // Save attachment metadata to database
        let attachment = sqlx::query_as::<_, Attachment>(
            "INSERT INTO attachments (id, task_id, comment_id, user_id, filename, original_filename, file_path, file_size, mime_type, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING *"
        )
        .bind(file_id)
        .bind(Some(task_id))
        .bind(None::<Uuid>)
        .bind(auth.user_id)
        .bind(&filename)
        .bind(&original_filename)
        .bind(&file_path)
        .bind(file_size)
        .bind(&mime_type)
        .bind(Utc::now())
        .fetch_one(pool.as_ref())
        .await?;

        attachments.push(AttachmentResponse::from(attachment));
    }

    Ok(HttpResponse::Created().json(attachments))
}

/// List all attachments for a task.
///
/// Returns attachment metadata for all files attached to the task.
#[utoipa::path(
    get,
    path = "/api/tasks/{task_id}/attachments",
    params(
        ("task_id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "List of attachments", body = Vec<AttachmentResponse>),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Attachments",
    security(("bearer_auth" = []))
)]
pub async fn list_attachments(
    pool: web::Data<DbPool>,
    _auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    let attachments = sqlx::query_as::<_, Attachment>(
        "SELECT * FROM attachments WHERE task_id = $1 ORDER BY created_at DESC"
    )
    .bind(task_id)
    .fetch_all(pool.as_ref())
    .await?;

    let responses: Vec<AttachmentResponse> = attachments
        .into_iter()
        .map(AttachmentResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(responses))
}

/// Query parameter for token-based auth (used by img tags)
#[derive(Debug, serde::Deserialize)]
pub struct TokenQuery {
    pub token: Option<String>,
}

/// Download an attachment file.
///
/// Returns the file content with appropriate headers for download.
/// Supports auth via Authorization header or `?token=` query parameter.
#[utoipa::path(
    get,
    path = "/api/attachments/{id}",
    params(
        ("id" = Uuid, Path, description = "Attachment ID")
    ),
    responses(
        (status = 200, description = "File download", content_type = "application/octet-stream"),
        (status = 401, description = "Not authenticated or access denied"),
        (status = 404, description = "Attachment not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Attachments",
    security(("bearer_auth" = []))
)]
pub async fn download_attachment(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    query: web::Query<TokenQuery>,
) -> AppResult<actix_files::NamedFile> {
    let config = Config::from_env();

    // Authenticate from Authorization header or ?token= query param
    let user_id = if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_str = auth_header.to_str()
            .map_err(|_| AppError::AuthenticationError("Invalid authorization header".to_string()))?;
        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::AuthenticationError("Invalid authorization format".to_string()));
        }
        let claims = utils::verify_token(&auth_str[7..], &config.jwt_secret)?;
        claims.user_id().map_err(|_| AppError::AuthenticationError("Invalid user ID in token".to_string()))?
    } else if let Some(ref token) = query.token {
        let claims = utils::verify_token(token, &config.jwt_secret)?;
        claims.user_id().map_err(|_| AppError::AuthenticationError("Invalid user ID in token".to_string()))?
    } else {
        return Err(AppError::AuthenticationError("Missing authorization".to_string()));
    };

    let attachment_id = path.into_inner();

    let attachment = sqlx::query_as::<_, Attachment>(
        "SELECT * FROM attachments WHERE id = $1"
    )
    .bind(attachment_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

    // Verify user has access to this attachment
    if attachment.user_id != user_id {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }

    let file = actix_files::NamedFile::open(&attachment.file_path)
        .map_err(|e| AppError::InternalError(format!("Failed to open file: {}", e)))?
        .set_content_disposition(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Inline,
            parameters: vec![actix_web::http::header::DispositionParam::Filename(attachment.original_filename)],
        });

    Ok(file)
}

/// Upload file attachments to a comment.
///
/// Accepts multipart/form-data file uploads attached to a specific comment.
#[utoipa::path(
    post,
    path = "/api/tasks/{task_id}/comments/{comment_id}/attachments",
    params(
        ("task_id" = Uuid, Path, description = "Task ID"),
        ("comment_id" = Uuid, Path, description = "Comment ID")
    ),
    request_body(content_type = "multipart/form-data", description = "File upload"),
    responses(
        (status = 201, description = "Files uploaded successfully", body = Vec<AttachmentResponse>),
        (status = 400, description = "Invalid request or file too large"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Attachments",
    security(("bearer_auth" = []))
)]
pub async fn upload_comment_attachment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    mut payload: Multipart,
) -> AppResult<HttpResponse> {
    let (task_id, comment_id) = path.into_inner();

    // Verify comment exists and user has access
    let _comment = sqlx::query!(
        "SELECT c.* FROM comments c
         JOIN tasks t ON c.task_id = t.id
         WHERE c.id = $1 AND c.task_id = $2 AND t.user_id = $3",
        comment_id,
        task_id,
        auth.user_id
    )
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    // Create upload directory if it doesn't exist
    std::fs::create_dir_all(UPLOAD_DIR)
        .map_err(|e| AppError::InternalError(format!("Failed to create upload directory: {}", e)))?;

    let mut attachments = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| AppError::InternalError(e.to_string()))?;

        let original_filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            .ok_or_else(|| AppError::ValidationError("Filename is required".to_string()))?;

        let mime_type = field.content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let filename = format!("{}-{}", file_id, original_filename);
        let file_path = format!("{}/{}", UPLOAD_DIR, filename);

        // Save file
        let mut file = std::fs::File::create(&file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to create file: {}", e)))?;

        let mut file_size: i64 = 0;

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| AppError::InternalError(e.to_string()))?;

            file_size += data.len() as i64;

            if file_size > MAX_FILE_SIZE as i64 {
                // Delete the partially written file
                let _ = std::fs::remove_file(&file_path);
                return Err(AppError::ValidationError(
                    "File size exceeds maximum limit of 10MB".to_string(),
                ));
            }

            file.write_all(&data)
                .map_err(|e| AppError::InternalError(format!("Failed to write file: {}", e)))?;
        }

        // Save attachment metadata to database (linked to comment)
        let attachment = sqlx::query_as::<_, Attachment>(
            "INSERT INTO attachments (id, task_id, comment_id, user_id, filename, original_filename, file_path, file_size, mime_type, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING *"
        )
        .bind(file_id)
        .bind(Some(task_id))
        .bind(Some(comment_id))
        .bind(auth.user_id)
        .bind(&filename)
        .bind(&original_filename)
        .bind(&file_path)
        .bind(file_size)
        .bind(&mime_type)
        .bind(Utc::now())
        .fetch_one(pool.as_ref())
        .await?;

        attachments.push(AttachmentResponse::from(attachment));
    }

    Ok(HttpResponse::Created().json(attachments))
}

/// Delete an attachment.
///
/// Permanently deletes an attachment and its associated file.
#[utoipa::path(
    delete,
    path = "/api/attachments/{id}",
    params(
        ("id" = Uuid, Path, description = "Attachment ID")
    ),
    responses(
        (status = 204, description = "Attachment deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Attachment not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Attachments",
    security(("bearer_auth" = []))
)]
pub async fn delete_attachment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let attachment_id = path.into_inner();

    let attachment = sqlx::query_as::<_, Attachment>(
        "SELECT * FROM attachments WHERE id = $1 AND user_id = $2"
    )
    .bind(attachment_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

    // Delete from database
    sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(attachment_id)
        .execute(pool.as_ref())
        .await?;

    // Delete file from filesystem
    if let Err(e) = std::fs::remove_file(&attachment.file_path) {
        log::warn!("Failed to delete file {}: {}", attachment.file_path, e);
    }

    Ok(HttpResponse::NoContent().finish())
}
