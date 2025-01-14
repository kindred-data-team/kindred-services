use actix_web::{web, Scope};
use crate::api::auth::{get_session, login_user, register_user};
use crate::api::rbac::handle_rbac;
pub fn auth_routes() -> Scope {
    web::scope("/api")
        .route("/auth/session/get-sessions", web::get().to(get_session))
        .route("/auth/register", web::post().to(register_user))
        .route("/auth/login", web::post().to(login_user))
        .route("/rbac", web::post().to(handle_rbac))
}