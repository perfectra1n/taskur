use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Attachment {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub download_url: String,
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
