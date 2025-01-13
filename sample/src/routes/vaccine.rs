use actix_web::{web, Scope};
use crate::api::vaccine::{create_vaccine, get_vaccine, get_vaccines, update_vaccines, delete_vaccines};
use sqlx::MySqlPool;
pub fn vaccine_routes(pool: web::Data<MySqlPool>) -> Scope {
    web::scope("/api/vaccine")
        .app_data(pool.clone()) 
        .route("", web::post().to(create_vaccine ))
        .route("", web::get().to(get_vaccines))
        .route("/{id}", web::get().to(get_vaccine))
        .route("/{id}", web::put().to(update_vaccines))
        .route("/{id}", web::delete().to(delete_vaccines))
}