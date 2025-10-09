use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TeamMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateTeamMemberRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
}
