use actix_web::{web, Scope};
use crate::api::auth::{create_session, get_session, login_user, register_user};
pub fn auth_routes() -> Scope {
    web::scope("/api/auth")
        .route("/session/create", web::post().to(create_session))
        .route("/session/get-sessions", web::get().to(get_session))
        .route("/register", web::post().to(register_user))
        .route("/login", web::post().to(login_user))
}