use crate::{
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::AuthenticatedUser,
    models::{CreateTeamRequest, Team, TeamWithMembers, TeamMemberWithUser, AddTeamMemberRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn list_teams(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let teams = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE owner_id = $1 OR id IN (SELECT team_id FROM team_members WHERE user_id = $1) ORDER BY created_at DESC"
    )
    .bind(auth.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(teams))
}

pub async fn create_team(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    body: web::Json<CreateTeamRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let team = sqlx::query_as::<_, Team>(
        "INSERT INTO teams (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&body.name)
    .bind(&body.description)
    .bind(auth.user_id)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(team))
}

pub async fn get_team(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let team_id = path.into_inner();

    let team = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE id = $1 AND (owner_id = $2 OR id IN (SELECT team_id FROM team_members WHERE user_id = $2))"
    )
    .bind(team_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

    // Get team members
    let members = sqlx::query_as::<_, TeamMemberWithUser>(
        "SELECT tm.user_id, u.email, tm.role, tm.joined_at
         FROM team_members tm
         JOIN users u ON tm.user_id = u.id
         WHERE tm.team_id = $1"
    )
    .bind(team_id)
    .fetch_all(pool.as_ref())
    .await?;

    let team_with_members = TeamWithMembers { team, members };

    Ok(HttpResponse::Ok().json(team_with_members))
}

pub async fn add_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<AddTeamMemberRequest>,
) -> AppResult<HttpResponse> {
    let team_id = path.into_inner();

    // Verify user owns the team
    let team = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE id = $1 AND owner_id = $2"
    )
    .bind(team_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::AuthenticationError("Only team owner can add members".to_string()))?;

    // Add member
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, joined_at)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(team_id)
    .bind(body.user_id)
    .bind(body.role.as_ref().unwrap_or(&"member".to_string()))
    .bind(Utc::now())
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Member added successfully"
    })))
}

pub async fn remove_team_member(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> AppResult<HttpResponse> {
    let (team_id, user_id) = path.into_inner();

    // Verify user owns the team
    let _team = sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE id = $1 AND owner_id = $2"
    )
    .bind(team_id)
    .bind(auth.user_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| AppError::AuthenticationError("Only team owner can remove members".to_string()))?;

    // Remove member
    let result = sqlx::query(
        "DELETE FROM team_members WHERE team_id = $1 AND user_id = $2"
    )
    .bind(team_id)
    .bind(user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Team member not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_team(
    pool: web::Data<DbPool>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let team_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM teams WHERE id = $1 AND owner_id = $2"
    )
    .bind(team_id)
    .bind(auth.user_id)
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Team not found or not authorized".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}

// Get all users (for assignment autocomplete)
pub async fn list_users(
    pool: web::Data<DbPool>,
    _auth: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let users = sqlx::query!(
        "SELECT id, email, created_at FROM users ORDER BY email"
    )
    .fetch_all(pool.as_ref())
    .await?;

    let user_list: Vec<_> = users.iter().map(|u| serde_json::json!({
        "id": u.id,
        "email": u.email,
        "created_at": u.created_at
    })).collect();

    Ok(HttpResponse::Ok().json(user_list))
}
