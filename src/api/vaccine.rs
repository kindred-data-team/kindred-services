use crate::models::vaccine::Vaccine;
use crate::repository::vaccine::{insert_vaccine, get_vaccine_by_id, get_all_vaccines, update_vaccine, delete_vaccine};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use crate::models::response::ApiResponse;

pub async fn create_vaccine(pool: web::Data<Pool<MySql>>, vaccine: web::Json<Vaccine>) -> impl Responder {
    match insert_vaccine(pool.get_ref(), &vaccine).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("Vaccine created successfully.")),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to create vaccine.")),
    }
}

pub async fn get_vaccine(pool: web::Data<Pool<MySql>>, vaccine_id: web::Path<i32>) -> impl Responder {
    match get_vaccine_by_id(pool.get_ref(), vaccine_id.into_inner()).await {
        Ok(vaccine) => HttpResponse::Ok().json(vaccine),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::new("Vaccine not found.")),
    }
}

pub async fn get_vaccines(pool: web::Data<Pool<MySql>>) -> impl Responder {
    match get_all_vaccines(pool.get_ref()).await {
        Ok(vaccines) => HttpResponse::Ok().json(vaccines),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch vaccines.")),
    }
}

pub async fn update_vaccines(pool: web::Data<Pool<MySql>>, vaccine_id: web::Path<i32>, vaccine: web::Json<Vaccine>) -> impl Responder {
    match update_vaccine(pool.get_ref(), vaccine_id.into_inner(), &vaccine).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("Vaccine updated successfully.")),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to update vaccine.")),
    }
}

pub async fn delete_vaccines(pool: web::Data<Pool<MySql>>, vaccine_id: web::Path<i32>) -> impl Responder {
    match delete_vaccine(pool.get_ref(), vaccine_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("Vaccine deleted successfully.")),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to delete vaccine.")),
    }
}