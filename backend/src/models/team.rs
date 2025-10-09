use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateTeamRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TeamUserAssociation {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddTeamMemberRequest {
    pub user_id: Uuid,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamWithMembers {
    #[serde(flatten)]
    pub team: Team,
    pub members: Vec<TeamMemberWithUser>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TeamMemberWithUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}
