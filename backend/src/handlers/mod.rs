mod auth;
mod tasks;
mod lists;
mod tags;
mod comments;
mod attachments;
mod team_members;
mod teams;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login))
                    .route("/me", web::get().to(auth::get_current_user))
            )
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(tasks::list_tasks))
                    .route("", web::post().to(tasks::create_task))
                    .route("/{id}", web::get().to(tasks::get_task))
                    .route("/{id}", web::put().to(tasks::update_task))
                    .route("/{id}", web::delete().to(tasks::delete_task))
                    .route("/{task_id}/comments", web::get().to(comments::list_comments))
                    .route("/{task_id}/comments", web::post().to(comments::create_comment))
                    .route("/{task_id}/comments/{comment_id}", web::put().to(comments::update_comment))
                    .route("/{task_id}/comments/{comment_id}", web::delete().to(comments::delete_comment))
                    .route("/{task_id}/attachments", web::get().to(attachments::list_attachments))
                    .route("/{task_id}/attachments", web::post().to(attachments::upload_attachment))
            )
            .service(
                web::scope("/lists")
                    .route("", web::get().to(lists::list_lists))
                    .route("", web::post().to(lists::create_list))
                    .route("/{id}", web::get().to(lists::get_list))
                    .route("/{id}", web::put().to(lists::update_list))
                    .route("/{id}", web::delete().to(lists::delete_list))
            )
            .service(
                web::scope("/tags")
                    .route("", web::get().to(tags::list_tags))
                    .route("", web::post().to(tags::create_tag))
                    .route("/{id}", web::delete().to(tags::delete_tag))
            )
            .service(
                web::scope("/attachments")
                    .route("/{id}", web::get().to(attachments::download_attachment))
                    .route("/{id}", web::delete().to(attachments::delete_attachment))
            )
            .service(
                web::scope("/team-members")
                    .route("", web::get().to(team_members::list_team_members))
                    .route("", web::post().to(team_members::create_team_member))
                    .route("/{id}", web::get().to(team_members::get_team_member))
                    .route("/{id}", web::put().to(team_members::update_team_member))
                    .route("/{id}", web::delete().to(team_members::delete_team_member))
            )
            .service(
                web::scope("/teams")
                    .route("", web::get().to(teams::list_teams))
                    .route("", web::post().to(teams::create_team))
                    .route("/{id}", web::get().to(teams::get_team))
                    .route("/{id}", web::delete().to(teams::delete_team))
                    .route("/{id}/members", web::post().to(teams::add_team_member))
                    .route("/{team_id}/members/{user_id}", web::delete().to(teams::remove_team_member))
            )
            .service(
                web::scope("/users")
                    .route("", web::get().to(teams::list_users))
            )
    );
}
