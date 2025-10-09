use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateTaskRequest, Task, TaskFilter, TaskPriority, TaskStatus, UnifiedSearchResult, UpdateTaskRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use utoipa;
use uuid::Uuid;
use validator::Validate;

/// List all tasks for the authenticated user.
///
/// Returns a list of tasks with optional filtering by status, priority, tags,
/// and list membership. Supports search via trigram similarity.
///
/// # Errors
///
/// Returns an error if database operation fails.
#[utoipa::path(
    get,
    path = "/api/tasks",
    params(TaskFilter),
    responses(
        (status = 200, description = "List of tasks", body = Vec<Task>),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tasks",
    security(("bearer_auth" = []))
)]
pub async fn list_tasks(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    query: web::Query<TaskFilter>,
) -> AppResult<HttpResponse> {
    // If there's a search query, use the trigram-based search function
    if let Some(ref search_query) = query.search {
        let limit = query.limit.unwrap_or(50);

        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM search_tasks($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(auth.user_id)
        .bind(search_query)
        .bind(&query.status)
        .bind(&query.priority)
        .bind(&query.tag)
        .bind(&query.list_id)
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?;

        return Ok(HttpResponse::Ok().json(tasks));
    }

    // Standard filtering without search
    let mut sql = "SELECT * FROM tasks WHERE user_id = $1".to_string();
    let mut params_count = 1;

    if query.status.is_some() {
        params_count += 1;
        sql.push_str(&format!(" AND status = ${}", params_count));
    }

    if query.priority.is_some() {
        params_count += 1;
        sql.push_str(&format!(" AND priority = ${}", params_count));
    }

    if let Some(ref tag) = query.tag {
        params_count += 1;
        sql.push_str(&format!(" AND ${}::text = ANY(tags)", params_count));
    }

    if let Some(ref list_id) = query.list_id {
        params_count += 1;
        sql.push_str(&format!(
            " AND EXISTS (SELECT 1 FROM task_lists tl WHERE tl.task_id = tasks.id AND tl.list_id = ${})",
            params_count
        ));
    }

    sql.push_str(" ORDER BY position ASC, created_at DESC");

    let mut query_builder = sqlx::query_as::<_, Task>(&sql)
        .bind(auth.user_id);

    if let Some(status) = &query.status {
        query_builder = query_builder.bind(status);
    }

    if let Some(priority) = &query.priority {
        query_builder = query_builder.bind(priority);
    }

    if let Some(ref tag) = query.tag {
        query_builder = query_builder.bind(tag);
    }

    if let Some(ref list_id) = query.list_id {
        query_builder = query_builder.bind(list_id);
    }

    let tasks = query_builder
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(tasks))
}

/// Create a new task.
///
/// Creates a new task with the provided details and optionally associates
/// it with one or more lists.
///
/// # Errors
///
/// Returns an error if:
/// - Validation fails
/// - Database operation fails
#[utoipa::path(
    post,
    path = "/api/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created successfully", body = Task),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tasks",
    security(("bearer_auth" = []))
)]
pub async fn create_task(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    body: web::Json<CreateTaskRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Get next position
    let max_position: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(position) FROM tasks WHERE user_id = $1"
    )
    .bind(auth.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    let position = max_position.unwrap_or(0) + 1;

    let default_reminders = serde_json::json!([]);

    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, user_id, title, description, status, priority, due_date, start_date, end_date, hero_image_id, assigned_to, reminders, tags, position, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(&body.title)
    .bind(&body.description)
    .bind(body.status.as_ref().unwrap_or(&TaskStatus::Todo))
    .bind(body.priority.as_ref().unwrap_or(&TaskPriority::Medium))
    .bind(&body.due_date)
    .bind(&body.start_date)
    .bind(&body.end_date)
    .bind(&body.hero_image_id)
    .bind(body.assigned_to.as_ref().unwrap_or(&vec![]))
    .bind(body.reminders.as_ref().unwrap_or(&default_reminders))
    .bind(body.tags.as_ref().unwrap_or(&vec![]))
    .bind(position)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    // Associate with lists if provided
    if let Some(ref list_ids) = body.list_ids {
        for list_id in list_ids {
            sqlx::query(
                "INSERT INTO task_lists (task_id, list_id) VALUES ($1, $2)"
            )
            .bind(task.id)
            .bind(list_id)
            .execute(pool.as_ref())
            .await?;
        }
    }

    Ok(HttpResponse::Created().json(task))
}

/// Get a specific task by ID.
///
/// Retrieves detailed information about a single task.
///
/// # Errors
///
/// Returns an error if:
/// - Task not found
/// - User doesn't own the task
/// - Database operation fails
#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task details", body = Task),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tasks",
    security(("bearer_auth" = []))
)]
pub async fn get_task(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(HttpResponse::Ok().json(task))
}

/// Update an existing task.
///
/// Updates task properties. Only provided fields will be updated.
///
/// # Errors
///
/// Returns an error if:
/// - Task not found
/// - User doesn't own the task
/// - Database operation fails
#[utoipa::path(
    put,
    path = "/api/tasks/{id}",
    request_body = UpdateTaskRequest,
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task updated successfully", body = Task),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tasks",
    security(("bearer_auth" = []))
)]
pub async fn update_task(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTaskRequest>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    // Verify task ownership
    let existing_task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let updated_task = sqlx::query_as::<_, Task>(
        "UPDATE tasks
         SET title = COALESCE($1, title),
             description = COALESCE($2, description),
             status = COALESCE($3, status),
             priority = COALESCE($4, priority),
             due_date = COALESCE($5, due_date),
             start_date = COALESCE($6, start_date),
             end_date = COALESCE($7, end_date),
             hero_image_id = COALESCE($8, hero_image_id),
             assigned_to = COALESCE($9, assigned_to),
             reminders = COALESCE($10, reminders),
             tags = COALESCE($11, tags),
             position = COALESCE($12, position),
             updated_at = $13
         WHERE id = $14 AND user_id = $15
         RETURNING *"
    )
    .bind(&body.title)
    .bind(&body.description)
    .bind(&body.status)
    .bind(&body.priority)
    .bind(&body.due_date)
    .bind(&body.start_date)
    .bind(&body.end_date)
    .bind(&body.hero_image_id)
    .bind(&body.assigned_to)
    .bind(&body.reminders)
    .bind(&body.tags)
    .bind(&body.position)
    .bind(Utc::now())
    .bind(task_id)
    .bind(auth.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(updated_task))
}

/// Delete a task.
///
/// Permanently deletes a task and its associations.
///
/// # Errors
///
/// Returns an error if:
/// - Task not found
/// - User doesn't own the task
/// - Database operation fails
#[utoipa::path(
    delete,
    path = "/api/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Tasks",
    security(("bearer_auth" = []))
)]
pub async fn delete_task(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let task_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Task not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}

/// Unified search across all entities (tasks, lists, tags, comments).
///
/// This endpoint provides a comprehensive search that returns results from all
/// entity types, ranked by relevance using PostgreSQL trigram similarity.
///
/// # Query Parameters
/// - `q`: Search query string (required)
/// - `limit`: Maximum number of results (optional, default: 50)
///
/// # Returns
/// Array of search results with entity type, ID, title, description, and relevance score.
#[utoipa::path(
    get,
    path = "/api/search",
    params(UnifiedSearchQuery),
    responses(
        (status = 200, description = "Search results", body = Vec<UnifiedSearchResult>),
        (status = 401, description = "Not authenticated"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Search",
    security(("bearer_auth" = []))
)]
pub async fn unified_search(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    query: web::Query<UnifiedSearchQuery>,
) -> AppResult<HttpResponse> {
    if query.q.is_empty() {
        return Ok(HttpResponse::Ok().json(Vec::<UnifiedSearchResult>::new()));
    }

    let limit = query.limit.unwrap_or(50);

    let results = sqlx::query_as::<_, UnifiedSearchResult>(
        "SELECT * FROM unified_search($1, $2, $3)"
    )
    .bind(auth.user_id)
    .bind(&query.q)
    .bind(limit)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(results))
}

/// Query parameters for unified search
#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UnifiedSearchQuery {
    /// Search query string
    #[schema(example = "project")]
    pub q: String,
    /// Maximum number of results (default: 50)
    #[schema(example = 20)]
    pub limit: Option<i32>,
}
