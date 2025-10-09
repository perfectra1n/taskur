use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Tag database model for categorizing tasks.
///
/// Tags provide a flexible way to label and categorize tasks
/// across different lists.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Tag {
    /// Unique identifier for the tag
    pub id: Uuid,
    /// ID of the user who owns this tag
    pub user_id: Uuid,
    /// Name of the tag
    #[schema(example = "urgent")]
    pub name: String,
    /// Optional hex color code for the tag
    #[schema(example = "#ef4444")]
    pub color: Option<String>,
    /// Timestamp when the tag was created
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a new tag.
#[derive(Debug, Deserialize, validator::Validate, ToSchema)]
pub struct CreateTagRequest {
    /// Tag name (1-50 characters)
    #[validate(length(min = 1, max = 50, message = "Name must be 1-50 characters"))]
    #[schema(example = "urgent")]
    pub name: String,
    /// Optional hex color code
    #[schema(example = "#ef4444")]
    pub color: Option<String>,
}
