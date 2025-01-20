use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use sqlx::{MySql, Pool};
use crate::api::v1::repositories::time_slots_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::time_slots_model::{*};

pub async fn index(
    pool: web::Data<Pool<MySql>>,
    opts: web::Query<FilterOptions>
) -> impl Responder {
    let date = NaiveDate::parse_from_str(&opts.date, "%Y-%m-%d").unwrap();

    match get_all_time_slots(pool.get_ref(), opts.clinic_id, date).await {
        Ok(time_slots) => HttpResponse::Ok().json(time_slots),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch time slots.")),
    }
}