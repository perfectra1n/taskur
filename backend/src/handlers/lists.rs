use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateListRequest, List, UpdateListRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn list_lists(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let lists = sqlx::query_as::<_, List>(
        "SELECT * FROM lists WHERE user_id = $1 ORDER BY position ASC, created_at DESC"
    )
    .bind(auth.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(lists))
}

pub async fn create_list(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    body: web::Json<CreateListRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let max_position: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(position) FROM lists WHERE user_id = $1"
    )
    .bind(auth.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    let position = max_position.unwrap_or(0) + 1;

    let list = sqlx::query_as::<_, List>(
        "INSERT INTO lists (id, user_id, name, color, icon, position, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(&body.name)
    .bind(&body.color)
    .bind(&body.icon)
    .bind(position)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(list))
}

pub async fn get_list(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let list_id = path.into_inner();

    let list = sqlx::query_as::<_, List>(
        "SELECT * FROM lists WHERE id = $1 AND user_id = $2"
    )
    .bind(list_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("List not found".to_string()))?;

    Ok(HttpResponse::Ok().json(list))
}

pub async fn update_list(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<UpdateListRequest>,
) -> AppResult<HttpResponse> {
    let list_id = path.into_inner();

    let updated_list = sqlx::query_as::<_, List>(
        "UPDATE lists
         SET name = COALESCE($1, name),
             color = COALESCE($2, color),
             icon = COALESCE($3, icon),
             position = COALESCE($4, position),
             updated_at = $5
         WHERE id = $6 AND user_id = $7
         RETURNING *"
    )
    .bind(&body.name)
    .bind(&body.color)
    .bind(&body.icon)
    .bind(&body.position)
    .bind(Utc::now())
    .bind(list_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("List not found".to_string()))?;

    Ok(HttpResponse::Ok().json(updated_list))
}

pub async fn delete_list(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let list_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM lists WHERE id = $1 AND user_id = $2"
    )
    .bind(list_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("List not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}
