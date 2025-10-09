mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure.
///
/// This struct defines all API endpoints, schemas, and security definitions
/// for the Swagger UI documentation.
#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::get_current_user,
        // Task endpoints
        handlers::tasks::list_tasks,
        handlers::tasks::create_task,
        handlers::tasks::get_task,
        handlers::tasks::update_task,
        handlers::tasks::delete_task,
        handlers::tasks::unified_search,
        // List endpoints
        handlers::lists::list_lists,
        handlers::lists::create_list,
        handlers::lists::get_list,
        handlers::lists::update_list,
        handlers::lists::delete_list,
        // Tag endpoints
        handlers::tags::list_tags,
        handlers::tags::create_tag,
        handlers::tags::delete_tag,
        // Comment endpoints
        handlers::comments::list_comments,
        handlers::comments::create_comment,
        handlers::comments::update_comment,
        handlers::comments::delete_comment,
        // Attachment endpoints
        handlers::attachments::upload_attachment,
        handlers::attachments::list_attachments,
        handlers::attachments::download_attachment,
        handlers::attachments::upload_comment_attachment,
        handlers::attachments::delete_attachment,
    ),
    components(
        schemas(
            models::User,
            models::UserResponse,
            models::CreateUserRequest,
            models::LoginRequest,
            models::AuthResponse,
            models::Task,
            models::TaskStatus,
            models::TaskPriority,
            models::CreateTaskRequest,
            models::UpdateTaskRequest,
            models::TaskFilter,
            models::UnifiedSearchResult,
            models::List,
            models::CreateListRequest,
            models::UpdateListRequest,
            models::Tag,
            models::CreateTagRequest,
            models::Comment,
            models::CreateCommentRequest,
            models::UpdateCommentRequest,
            models::Attachment,
            models::AttachmentResponse,
            models::Team,
            models::CreateTeamRequest,
            models::UpdateTeamRequest,
            models::TeamUserAssociation,
            models::AddTeamMemberRequest,
            models::TeamWithMembers,
            models::TeamMemberWithUser,
            models::TeamMember,
            models::CreateTeamMemberRequest,
            models::UpdateTeamMemberRequest,
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication and registration endpoints"),
        (name = "Tasks", description = "Task management endpoints"),
        (name = "Lists", description = "List management endpoints"),
        (name = "Tags", description = "Tag management endpoints"),
        (name = "Comments", description = "Task comment endpoints"),
        (name = "Attachments", description = "File attachment endpoints"),
        (name = "Teams", description = "Team management endpoints"),
        (name = "Team Members", description = "Team member management endpoints"),
        (name = "Search", description = "Unified search across all entities"),
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Taskur API",
        version = "0.1.0",
        description = "A comprehensive task management API with authentication, collaboration, and file management features.",
        contact(
            name = "API Support",
            email = "support@taskur.dev"
        )
    )
)]
struct ApiDoc;

/// Security scheme modifier for OpenAPI.
///
/// Adds JWT bearer token authentication to the OpenAPI specification.
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            let mut http = utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            );
            http.bearer_format = Some("JWT".to_string());

            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(http),
            );
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env();
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let openapi = ApiDoc::openapi();

    info!("Starting server at {}:{}", config.host, config.port);
    info!("Swagger UI available at http://{}:{}/swagger-ui/", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            .configure(handlers::configure)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
