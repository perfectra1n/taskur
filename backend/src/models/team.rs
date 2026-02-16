use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// Team database model for collaboration.
///
/// Teams allow multiple users to collaborate on shared tasks
/// and projects with role-based access.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct Team {
    /// Unique identifier for the team
    pub id: Uuid,
    /// Name of the team
    #[schema(example = "Engineering Team")]
    pub name: String,
    /// Optional team description
    #[schema(example = "Backend development team")]
    pub description: Option<String>,
    /// ID of the user who owns/created the team
    pub owner_id: Uuid,
    /// Timestamp when the team was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the team was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new team.
#[derive(Debug, Deserialize, validator::Validate, ToSchema, TS)]
#[ts(export)]
pub struct CreateTeamRequest {
    /// Team name (1-255 characters)
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    #[schema(example = "Engineering Team")]
    pub name: String,
    /// Optional team description
    #[schema(example = "Backend development team")]
    #[ts(optional)]
    pub description: Option<String>,
}

/// Request payload for updating an existing team.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct UpdateTeamRequest {
    /// Updated team name
    #[schema(example = "Engineering Team")]
    #[ts(optional)]
    pub name: Option<String>,
    /// Updated team description
    #[schema(example = "Full-stack development team")]
    #[ts(optional)]
    pub description: Option<String>,
}

/// Team-User association model.
///
/// Represents the relationship between a team and its members,
/// including role information.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct TeamUserAssociation {
    /// Team ID
    pub team_id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// User's role in the team
    #[schema(example = "developer")]
    pub role: String,
    /// Timestamp when the user joined the team
    pub joined_at: DateTime<Utc>,
}

/// Request payload for adding a member to a team.
#[derive(Debug, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct AddTeamMemberRequest {
    /// ID of the user to add
    pub user_id: Uuid,
    /// Optional role for the user (defaults to 'member')
    #[schema(example = "developer")]
    #[ts(optional)]
    pub role: Option<String>,
}

/// Team with full member details response.
///
/// Contains team information along with all member details.
#[derive(Debug, Serialize, ToSchema)]
pub struct TeamWithMembers {
    /// Team information
    #[serde(flatten)]
    pub team: Team,
    /// List of team members with user details
    pub members: Vec<TeamMemberWithUser>,
}

/// Team member with user information.
///
/// Combines user details with team membership information.
#[derive(Debug, Serialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct TeamMemberWithUser {
    /// User ID
    pub user_id: Uuid,
    /// User's email address
    #[schema(example = "user@example.com")]
    pub email: String,
    /// User's role in the team
    #[schema(example = "developer")]
    pub role: String,
    /// When the user joined the team
    pub joined_at: DateTime<Utc>,
}
