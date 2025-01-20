use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use validator::Validate;
use serde_json::json;
use crate::api::v1::repositories::service_appointments_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::service_appointments_model::{*};
use crate::api::v1::middleware::authentication::Claims;

pub async fn index(
    pool: web::Data<Pool<MySql>>,
    opts: web::Query<FilterOptions>,
    req: HttpRequest
) -> impl Responder {
    let limit = if opts.limit.unwrap_or(10) == 100 {
        100
    } else {
        opts.limit.unwrap_or(10)
    };
    let page = opts.page.unwrap_or(1);

    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user_id = claims.sub.parse::<i32>().unwrap();

    match get_user_service_appointments(pool.get_ref(), page, limit, user_id).await {
        Ok(time_slots) => HttpResponse::Ok().json(time_slots),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch appointments.")),
    }
}

pub async fn show(
    pool: web::Data<Pool<MySql>>, 
    id: web::Path<i32>,
    req: HttpRequest
) -> impl Responder {

    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user_id = claims.sub.parse::<i32>().unwrap();

    match get_service_appointment_by_id(pool.get_ref(), id.into_inner(), user_id).await {
        Ok(product_type) => HttpResponse::Ok().json(product_type),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::new("Product type not found.")),
    }
}

pub async fn insert(
    pool: web::Data<Pool<MySql>>, 
    service_appointment: Result<web::Json<CreateServiceAppointment>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {

    // First handle JSON parsing errors
    let service_appointment = match service_appointment {
        Ok(service_appointment_json) => service_appointment_json,
        Err(err) => {
            return HttpResponse::BadRequest().json(json!({
                "name": [{
                    "code": "invalid_type",
                    "message": format!("Invalid JSON format: {}", err),
                    "params": {
                        "error": err.to_string()
                    }
                }]
            }));
        }
    };

    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user_id = claims.sub.parse::<i32>().unwrap();

    let service_appointment_data = service_appointment.into_inner();
    let is_valid = service_appointment_data.validate();

    match is_valid {
        Ok(_) => {
            match add_service_appointment(pool.get_ref(), &service_appointment_data, user_id).await {
                Ok(_) => return HttpResponse::Ok().json(ApiResponse::new("Successfully added")),
                Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(err)
        }
    }
}