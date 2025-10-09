use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, message = "Comment cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}
