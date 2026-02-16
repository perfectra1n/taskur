use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// Comment database model for task discussions.
///
/// Comments allow users to add notes, discussions, and updates
/// to tasks for collaboration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct Comment {
    /// Unique identifier for the comment
    pub id: Uuid,
    /// ID of the task this comment belongs to
    pub task_id: Uuid,
    /// ID of the user who created the comment
    pub user_id: Uuid,
    /// Comment content
    #[schema(example = "This task is blocked by issue #123")]
    pub content: String,
    /// Timestamp when the comment was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the comment was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new comment.
#[derive(Debug, Deserialize, validator::Validate, ToSchema, TS)]
#[ts(export)]
pub struct CreateCommentRequest {
    /// Comment content (minimum 1 character)
    #[validate(length(min = 1, message = "Comment cannot be empty"))]
    #[schema(example = "This task is blocked by issue #123")]
    pub content: String,
}

/// Request payload for updating an existing comment.
#[derive(Debug, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct UpdateCommentRequest {
    /// Updated comment content
    #[schema(example = "This task is no longer blocked")]
    pub content: String,
}
