use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{Attachment, AttachmentResponse},
};
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt;
use std::io::Write;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const UPLOAD_DIR: &str = "./uploads";

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

pub async fn download_attachment(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<actix_files::NamedFile> {
    let attachment_id = path.into_inner();

    let attachment = sqlx::query_as::<_, Attachment>(
        "SELECT * FROM attachments WHERE id = $1"
    )
    .bind(attachment_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

    // Verify user has access to this attachment
    if attachment.user_id != auth.user_id {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }

    let file = actix_files::NamedFile::open(&attachment.file_path)
        .map_err(|e| AppError::InternalError(format!("Failed to open file: {}", e)))?
        .set_content_disposition(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![actix_web::http::header::DispositionParam::Filename(attachment.original_filename)],
        });

    Ok(file)
}

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
