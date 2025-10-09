use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateTeamMemberRequest, TeamMember, UpdateTeamMemberRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn list_team_members(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let team_members = sqlx::query_as::<_, TeamMember>(
        "SELECT * FROM team_members WHERE user_id = $1 ORDER BY name ASC"
    )
    .bind(auth.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(team_members))
}

pub async fn create_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    body: web::Json<CreateTeamMemberRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let team_member = sqlx::query_as::<_, TeamMember>(
        "INSERT INTO team_members (id, user_id, name, email, avatar_url, role, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(&body.name)
    .bind(&body.email)
    .bind(&body.avatar_url)
    .bind(&body.role)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(team_member))
}

pub async fn get_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let member_id = path.into_inner();

    let team_member = sqlx::query_as::<_, TeamMember>(
        "SELECT * FROM team_members WHERE id = $1 AND user_id = $2"
    )
    .bind(member_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Team member not found".to_string()))?;

    Ok(HttpResponse::Ok().json(team_member))
}

pub async fn update_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTeamMemberRequest>,
) -> AppResult<HttpResponse> {
    let member_id = path.into_inner();

    // Verify ownership
    let _existing = sqlx::query_as::<_, TeamMember>(
        "SELECT * FROM team_members WHERE id = $1 AND user_id = $2"
    )
    .bind(member_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Team member not found".to_string()))?;

    let updated_member = sqlx::query_as::<_, TeamMember>(
        "UPDATE team_members
         SET name = COALESCE($1, name),
             email = COALESCE($2, email),
             avatar_url = COALESCE($3, avatar_url),
             role = COALESCE($4, role),
             updated_at = $5
         WHERE id = $6 AND user_id = $7
         RETURNING *"
    )
    .bind(&body.name)
    .bind(&body.email)
    .bind(&body.avatar_url)
    .bind(&body.role)
    .bind(Utc::now())
    .bind(member_id)
    .bind(auth.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(updated_member))
}

pub async fn delete_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let member_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM team_members WHERE id = $1 AND user_id = $2"
    )
    .bind(member_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Team member not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}
