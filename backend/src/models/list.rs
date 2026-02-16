use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// List database model for organizing tasks.
///
/// Lists allow users to group and organize their tasks into
/// categories with custom colors and icons.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct List {
    /// Unique identifier for the list
    pub id: Uuid,
    /// ID of the user who owns this list
    pub user_id: Uuid,
    /// Name of the list
    #[schema(example = "Work Projects")]
    pub name: String,
    /// Optional hex color code for the list
    #[schema(example = "#3b82f6")]
    pub color: Option<String>,
    /// Optional icon identifier for the list
    #[schema(example = "briefcase")]
    pub icon: Option<String>,
    /// Position for ordering lists
    pub position: i32,
    /// Timestamp when the list was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the list was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new list.
#[derive(Debug, Deserialize, validator::Validate, ToSchema, TS)]
#[ts(export)]
pub struct CreateListRequest {
    /// List name (1-100 characters)
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    #[schema(example = "Work Projects")]
    pub name: String,
    /// Optional hex color code
    #[schema(example = "#3b82f6")]
    #[ts(optional)]
    pub color: Option<String>,
    /// Optional icon identifier
    #[schema(example = "briefcase")]
    #[ts(optional)]
    pub icon: Option<String>,
}

/// Request payload for updating an existing list.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct UpdateListRequest {
    /// Updated list name
    #[schema(example = "Work Projects")]
    #[ts(optional)]
    pub name: Option<String>,
    /// Updated color
    #[schema(example = "#3b82f6")]
    #[ts(optional)]
    pub color: Option<String>,
    /// Updated icon
    #[schema(example = "briefcase")]
    #[ts(optional)]
    pub icon: Option<String>,
    /// Updated position
    #[ts(optional)]
    pub position: Option<i32>,
}
