use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Task status enumeration.
///
/// Represents the current state of a task in the workflow.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
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
#[derive(Debug, Deserialize, validator::Validate, ToSchema)]
pub struct CreateTaskRequest {
    /// Task title (1-500 characters)
    #[validate(length(min = 1, max = 500, message = "Title must be 1-500 characters"))]
    #[schema(example = "Complete project documentation")]
    pub title: String,
    /// Optional task description
    #[schema(example = "Write comprehensive API documentation")]
    pub description: Option<String>,
    /// Initial status (defaults to 'todo')
    pub status: Option<TaskStatus>,
    /// Task priority (defaults to 'medium')
    pub priority: Option<TaskPriority>,
    /// Optional due date
    pub due_date: Option<DateTime<Utc>>,
    /// Optional start date
    pub start_date: Option<DateTime<Utc>>,
    /// Optional end date
    pub end_date: Option<DateTime<Utc>>,
    /// Optional hero image ID
    pub hero_image_id: Option<Uuid>,
    /// Optional list of assigned user IDs
    pub assigned_to: Option<Vec<Uuid>>,
    /// Optional reminder configuration
    #[schema(value_type = Object)]
    pub reminders: Option<serde_json::Value>,
    /// Optional tags
    pub tags: Option<Vec<String>>,
    /// Optional list IDs to add the task to
    pub list_ids: Option<Vec<Uuid>>,
}

/// Request payload for updating an existing task.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskRequest {
    /// Updated task title
    #[schema(example = "Complete project documentation")]
    pub title: Option<String>,
    /// Updated task description
    pub description: Option<String>,
    /// Updated task status
    pub status: Option<TaskStatus>,
    /// Updated task priority
    pub priority: Option<TaskPriority>,
    /// Updated due date
    pub due_date: Option<DateTime<Utc>>,
    /// Updated start date
    pub start_date: Option<DateTime<Utc>>,
    /// Updated end date
    pub end_date: Option<DateTime<Utc>>,
    /// Updated hero image ID
    pub hero_image_id: Option<Uuid>,
    /// Updated assigned users
    pub assigned_to: Option<Vec<Uuid>>,
    /// Updated reminders
    #[schema(value_type = Object)]
    pub reminders: Option<serde_json::Value>,
    /// Updated tags
    pub tags: Option<Vec<String>>,
    /// Updated position
    pub position: Option<i32>,
}

/// Query parameters for filtering tasks.
///
/// All filters are optional and can be combined.
#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct TaskFilter {
    /// Filter by task status
    pub status: Option<TaskStatus>,
    /// Filter by task priority
    pub priority: Option<TaskPriority>,
    /// Filter by tag name
    pub tag: Option<String>,
    /// Filter by list ID
    pub list_id: Option<Uuid>,
    /// Search in title and description (uses PostgreSQL trigram similarity)
    pub search: Option<String>,
    /// Maximum number of results to return (default: 50)
    pub limit: Option<i32>,
}

/// Unified search result across all entity types.
///
/// Returns search results from tasks, lists, tags, and comments.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
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
