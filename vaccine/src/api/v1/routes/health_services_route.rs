use actix_web::{web, Scope};
use crate::api::v1::controllers::health_services_controller::{*};
use sqlx::MySqlPool;

pub fn health_services_routes(pool: web::Data<MySqlPool>) -> Scope {
    web::scope("/api/health_services")
        .app_data(pool.clone()) 
        .route("", web::post().to(insert))
        .route("/", web::get().to(index))
        .route("/{id}", web::get().to(show))
        .route("/{id}", web::patch().to(update))
        .route("/{id}", web::delete().to(delete))
}