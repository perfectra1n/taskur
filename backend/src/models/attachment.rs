use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// Attachment database model for file uploads.
///
/// Attachments can be associated with tasks or comments,
/// providing file storage and management capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Attachment {
    /// Unique identifier for the attachment
    pub id: Uuid,
    /// Optional ID of the task this attachment belongs to
    pub task_id: Option<Uuid>,
    /// Optional ID of the comment this attachment belongs to
    pub comment_id: Option<Uuid>,
    /// ID of the user who uploaded the attachment
    pub user_id: Uuid,
    /// Stored filename (may be different from original)
    #[schema(example = "abc123.pdf")]
    pub filename: String,
    /// Original filename from upload
    #[schema(example = "document.pdf")]
    pub original_filename: String,
    /// File path on storage
    #[schema(example = "uploads/abc123.pdf")]
    pub file_path: String,
    /// File size in bytes
    #[schema(example = 1024000)]
    pub file_size: i64,
    /// MIME type of the file
    #[schema(example = "application/pdf")]
    pub mime_type: String,
    /// Timestamp when the attachment was created
    pub created_at: DateTime<Utc>,
}

/// Public attachment information response.
///
/// Contains safe-to-expose attachment data with download URL.
#[derive(Debug, Serialize, ToSchema, TS)]
#[ts(export)]
pub struct AttachmentResponse {
    /// Unique identifier for the attachment
    pub id: Uuid,
    /// Optional task ID
    pub task_id: Option<Uuid>,
    /// Optional comment ID
    pub comment_id: Option<Uuid>,
    /// Original filename
    #[schema(example = "document.pdf")]
    pub original_filename: String,
    /// File size in bytes
    #[schema(example = 1024000)]
    #[ts(type = "number")]
    pub file_size: i64,
    /// MIME type
    #[schema(example = "application/pdf")]
    pub mime_type: String,
    /// URL to download the attachment
    #[schema(example = "/api/attachments/550e8400-e29b-41d4-a716-446655440000")]
    pub download_url: String,
    /// Timestamp when uploaded
    pub created_at: DateTime<Utc>,
}

impl From<Attachment> for AttachmentResponse {
    fn from(attachment: Attachment) -> Self {
        Self {
            id: attachment.id,
            task_id: attachment.task_id,
            comment_id: attachment.comment_id,
            original_filename: attachment.original_filename,
            file_size: attachment.file_size,
            mime_type: attachment.mime_type,
            download_url: format!("/api/attachments/{}", attachment.id),
            created_at: attachment.created_at,
        }
    }
}
