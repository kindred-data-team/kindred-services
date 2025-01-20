use actix_web::{web, Scope};
use crate::api::v1::controllers::cities_controller::{*};
use sqlx::MySqlPool;

pub fn cities_routes(pool: web::Data<MySqlPool>) -> Scope {
    web::scope("/api/cities")
        .app_data(pool.clone()) 
        .route("", web::post().to(insert))
        .route("/", web::get().to(index))
        .route("/{id}", web::get().to(show))
        .route("/{id}", web::delete().to(delete))
}