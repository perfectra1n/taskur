use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    #[serde(rename = "inprogress")]
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub hero_image_id: Option<Uuid>,
    pub assigned_to: Vec<Uuid>,
    pub reminders: serde_json::Value,
    pub tags: Vec<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 500, message = "Title must be 1-500 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub hero_image_id: Option<Uuid>,
    pub assigned_to: Option<Vec<Uuid>>,
    pub reminders: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub list_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub hero_image_id: Option<Uuid>,
    pub assigned_to: Option<Vec<Uuid>>,
    pub reminders: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub tag: Option<String>,
    pub list_id: Option<Uuid>,
    pub search: Option<String>,
}
