use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

/// Task status enumeration.
///
/// Represents the current state of a task in the workflow.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, TS)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum TaskStatus {
    /// Task has not been started
    Todo,
    /// Task is currently being worked on
    #[serde(rename = "inprogress")]
    InProgress,
    /// Task has been finished
    Completed,
}

/// Task priority enumeration.
///
/// Indicates the urgency and importance level of a task.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, TS)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum TaskPriority {
    /// Low priority task
    Low,
    /// Medium priority task
    Medium,
    /// High priority task
    High,
    /// Urgent task requiring immediate attention
    Urgent,
}

/// Task database model.
///
/// Represents a complete task with all its properties including
/// scheduling, assignment, and categorization information.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct Task {
    /// Unique identifier for the task
    pub id: Uuid,
    /// ID of the user who created the task
    pub user_id: Uuid,
    /// Task title
    #[schema(example = "Complete project documentation")]
    pub title: String,
    /// Detailed task description
    #[schema(example = "Write comprehensive API documentation")]
    pub description: Option<String>,
    /// Current status of the task
    pub status: TaskStatus,
    /// Priority level of the task
    pub priority: TaskPriority,
    /// Task deadline
    pub due_date: Option<DateTime<Utc>>,
    /// When the task should start
    pub start_date: Option<DateTime<Utc>>,
    /// When the task was completed
    pub end_date: Option<DateTime<Utc>>,
    /// Optional hero image attachment ID
    pub hero_image_id: Option<Uuid>,
    /// List of user IDs assigned to this task
    pub assigned_to: Vec<Uuid>,
    /// Reminder configuration (JSON)
    #[schema(value_type = Object)]
    pub reminders: serde_json::Value,
    /// Tags associated with the task
    pub tags: Vec<String>,
    /// Position for ordering tasks in a list
    pub position: i32,
    /// Timestamp when the task was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the task was last updated
    pub updated_at: DateTime<Utc>,
    /// Search relevance score (only populated for search results)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance: Option<f32>,
}

/// Request payload for creating a new task.
///
/// Most fields are optional with sensible defaults applied.
#[derive(Debug, Deserialize, validator::Validate, ToSchema, TS)]
#[ts(export)]
pub struct CreateTaskRequest {
    /// Task title (1-500 characters)
    #[validate(length(min = 1, max = 500, message = "Title must be 1-500 characters"))]
    #[schema(example = "Complete project documentation")]
    pub title: String,
    /// Optional task description
    #[schema(example = "Write comprehensive API documentation")]
    #[ts(optional)]
    pub description: Option<String>,
    /// Initial status (defaults to 'todo')
    #[ts(optional)]
    pub status: Option<TaskStatus>,
    /// Task priority (defaults to 'medium')
    #[ts(optional)]
    pub priority: Option<TaskPriority>,
    /// Optional due date
    #[ts(optional)]
    pub due_date: Option<DateTime<Utc>>,
    /// Optional start date
    #[ts(optional)]
    pub start_date: Option<DateTime<Utc>>,
    /// Optional end date
    #[ts(optional)]
    pub end_date: Option<DateTime<Utc>>,
    /// Optional hero image ID
    #[ts(optional)]
    pub hero_image_id: Option<Uuid>,
    /// Optional list of assigned user IDs
    #[ts(optional)]
    pub assigned_to: Option<Vec<Uuid>>,
    /// Optional reminder configuration
    #[schema(value_type = Object)]
    #[ts(optional)]
    pub reminders: Option<serde_json::Value>,
    /// Optional tags
    #[ts(optional)]
    pub tags: Option<Vec<String>>,
    /// Optional list IDs to add the task to
    #[ts(optional)]
    pub list_ids: Option<Vec<Uuid>>,
}

/// Request payload for updating an existing task.
///
/// All fields are optional - only provided fields will be updated.
/// For clearable fields (description, dates, hero_image_id), we use
/// `Option<Option<T>>` where:
/// - `None` (key absent from JSON) = field not provided, keep existing value
/// - `Some(None)` (key present with `null`) = explicitly clear the field
/// - `Some(Some(v))` (key present with value) = set to new value
#[derive(Debug, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct UpdateTaskRequest {
    /// Updated task title
    #[schema(example = "Complete project documentation")]
    #[ts(optional)]
    pub title: Option<String>,
    /// Updated task description (nullable — send null to clear)
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[schema(nullable, value_type = Option<String>)]
    #[ts(optional, type = "string | null")]
    pub description: Option<Option<String>>,
    /// Updated task status
    #[ts(optional)]
    pub status: Option<TaskStatus>,
    /// Updated task priority
    #[ts(optional)]
    pub priority: Option<TaskPriority>,
    /// Updated due date (nullable — send null to clear)
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[schema(nullable, value_type = Option<String>)]
    #[ts(optional, type = "string | null")]
    pub due_date: Option<Option<DateTime<Utc>>>,
    /// Updated start date (nullable — send null to clear)
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[schema(nullable, value_type = Option<String>)]
    #[ts(optional, type = "string | null")]
    pub start_date: Option<Option<DateTime<Utc>>>,
    /// Updated end date (nullable — send null to clear)
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[schema(nullable, value_type = Option<String>)]
    #[ts(optional, type = "string | null")]
    pub end_date: Option<Option<DateTime<Utc>>>,
    /// Updated hero image ID (nullable — send null to clear)
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[schema(nullable, value_type = Option<String>)]
    #[ts(optional, type = "string | null")]
    pub hero_image_id: Option<Option<Uuid>>,
    /// Updated assigned users
    #[ts(optional)]
    pub assigned_to: Option<Vec<Uuid>>,
    /// Updated reminders
    #[schema(value_type = Object)]
    #[ts(optional)]
    pub reminders: Option<serde_json::Value>,
    /// Updated tags
    #[ts(optional)]
    pub tags: Option<Vec<String>>,
    /// Updated position
    #[ts(optional)]
    pub position: Option<i32>,
}

/// Deserializes a double-option field from JSON.
/// Missing key → `None`, explicit `null` → `Some(None)`, value → `Some(Some(v))`.
fn deserialize_double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

/// Query parameters for filtering tasks.
///
/// All filters are optional and can be combined.
#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams, TS)]
#[ts(export)]
pub struct TaskFilter {
    /// Filter by task status
    #[ts(optional)]
    pub status: Option<TaskStatus>,
    /// Filter by task priority
    #[ts(optional)]
    pub priority: Option<TaskPriority>,
    /// Filter by tag name
    #[ts(optional)]
    pub tag: Option<String>,
    /// Filter by list ID
    #[ts(optional)]
    pub list_id: Option<Uuid>,
    /// Search in title and description (uses PostgreSQL trigram similarity)
    #[ts(optional)]
    pub search: Option<String>,
    /// Maximum number of results to return (default: 50)
    #[ts(optional)]
    pub limit: Option<i32>,
}

/// Unified search result across all entity types.
///
/// Returns search results from tasks, lists, tags, and comments.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct UnifiedSearchResult {
    /// Type of entity (task, list, tag, comment)
    pub entity_type: String,
    /// Unique identifier for the entity
    pub entity_id: Uuid,
    /// Primary text (title, name, etc.)
    pub title: String,
    /// Secondary text (description, content, etc.)
    pub description: String,
    /// Search relevance score (0.0 to 1.0+)
    pub relevance: f32,
}
