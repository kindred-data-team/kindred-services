use actix_web::{web, Scope};
use crate::api::v1::controllers::{*};
use sqlx::MySqlPool;

pub fn routes(pool: web::Data<MySqlPool>) -> Scope {
    web::scope("/api/v1")
        .app_data(pool.clone()) 
        .service(
            web::scope("/product_types")
            .route("", web::post().to(product_types_controller::insert))
            .route("/", web::get().to(product_types_controller::index))
            .route("/{id}", web::get().to(product_types_controller::show))
            .route("/{id}", web::patch().to(product_types_controller::update))
            .route("/{id}", web::delete().to(product_types_controller::delete))
            .service(
                web::scope("/services")
                    .route("", web::post().to(health_services::insert))
                    .route("/", web::get().to(health_services::index))
                    .route("/{id}", web::get().to(health_services::show))
                    .route("/{id}", web::patch().to(health_services::update))
                    .route("/{id}", web::delete().to(health_services::delete))
                    .service(
                        web::scope("/variants")
                        .route("", web::post().to(health_service_variants_controller::insert))
                        .route("/", web::get().to(health_service_variants_controller::index))
                        .route("/{id}", web::get().to(health_service_variants_controller::show))
                        .route("/{id}", web::patch().to(health_service_variants_controller::update))
                        .route("/{id}", web::delete().to(health_service_variants_controller::delete))
                    )
            )
        )
        .service(
            web::scope("/clinics")
            .service(
                web::scope("/services")
                .route("", web::post().to(clinic_services_controller::insert))
                .route("/", web::get().to(clinic_services_controller::index))
                .route("/{id}", web::patch().to(clinic_services_controller::update))
                .route("/{id}", web::delete().to(clinic_services_controller::delete))
            )
            .service(
                web::scope("/time_slots")
                .route("/", web::get().to(time_slots_controller::index))
            )
        )
        .service(
            web::scope("/appointments")
            .route("", web::post().to(service_appointments_controller::insert))
            .route("/", web::get().to(service_appointments_controller::index))
            .route("/{id}", web::get().to(service_appointments_controller::show))
        )
    }