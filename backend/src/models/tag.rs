use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateTagRequest {
    #[validate(length(min = 1, max = 50, message = "Name must be 1-50 characters"))]
    pub name: String,
    pub color: Option<String>,
}
