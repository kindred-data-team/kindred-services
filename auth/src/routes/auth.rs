use actix_web::{web, Scope};

use crate::api::laravel::default_handler;
use crate::api::rbac::handle_rbac;
use crate::api::auth::{get_session, login_user, refresh_token, register_user, reset_password, social_login};

pub fn auth_routes() -> Scope {
    web::scope("/api")
        .route("/auth/session/get-sessions", web::get().to(get_session))
        .route("/auth/register", web::post().to(register_user))
        .route("/auth/login", web::post().to(login_user))
        .route("/auth/refresh", web::post().to(refresh_token))
        .route("/auth/reset-password", web::post().to(reset_password))
        .route("/social-auth/{provider}/login", web::post().to(social_login))
        .route("/rbac", web::post().to(handle_rbac))
        .default_service(web::to(default_handler))
}