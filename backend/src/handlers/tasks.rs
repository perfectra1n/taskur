use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateTaskRequest, Task, TaskFilter, TaskPriority, TaskStatus, UpdateTaskRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn list_tasks(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    query: web::Query<TaskFilter>,
) -> AppResult<HttpResponse> {
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

    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    if search_pattern.is_some() {
        params_count += 1;
        sql.push_str(&format!(
            " AND (title ILIKE ${} OR description ILIKE ${})",
            params_count, params_count
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

    if let Some(ref pattern) = search_pattern {
        query_builder = query_builder.bind(pattern);
    }

    let tasks = query_builder
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(tasks))
}

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
