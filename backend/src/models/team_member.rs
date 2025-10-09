use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Team member database model.
///
/// Represents a team member with profile information
/// and role assignment.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TeamMember {
    /// Unique identifier for the team member
    pub id: Uuid,
    /// Associated user ID
    pub user_id: Uuid,
    /// Display name of the team member
    #[schema(example = "John Doe")]
    pub name: String,
    /// Optional email address
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    /// Optional avatar image URL
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,
    /// Optional role description
    #[schema(example = "Senior Developer")]
    pub role: Option<String>,
    /// Timestamp when the team member was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the team member was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new team member.
#[derive(Debug, Deserialize, validator::Validate, ToSchema)]
pub struct CreateTeamMemberRequest {
    /// Display name (1-255 characters)
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    #[schema(example = "John Doe")]
    pub name: String,
    /// Optional email address (must be valid format)
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    /// Optional avatar URL
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,
    /// Optional role description
    #[schema(example = "Senior Developer")]
    pub role: Option<String>,
}

/// Request payload for updating an existing team member.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTeamMemberRequest {
    /// Updated name
    #[schema(example = "John Doe")]
    pub name: Option<String>,
    /// Updated email
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    /// Updated avatar URL
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,
    /// Updated role
    #[schema(example = "Lead Developer")]
    pub role: Option<String>,
}
