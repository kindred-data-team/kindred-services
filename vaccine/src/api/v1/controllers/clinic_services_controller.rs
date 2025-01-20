use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use crate::api::v1::repositories::clinic_services_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::clinic_services_model::{*};
use crate::api::v1::middleware::authentication::Claims;
use validator::Validate;
use serde_json::json;

pub async fn index(
    pool: web::Data<Pool<MySql>>,
    opts: web::Query<FilterOptions>
) -> impl Responder {
    let limit = if opts.limit.unwrap_or(10) == 100 {
        100
    } else {
        opts.limit.unwrap_or(10)
    };
    let page = (opts.page.unwrap_or(1) - 1) * limit;
    let service_id = opts.service_id;

    match get_clinic_services(pool.get_ref(), page, limit, service_id).await {
        Ok(clinic_services) => HttpResponse::Ok().json(clinic_services),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch clinic services.")),
    }
}

pub async fn insert(
    pool: web::Data<Pool<MySql>>, 
    clinic_service: Result<web::Json<CreateClinicService>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {

    // First handle JSON parsing errors
    let clinic_service = match clinic_service {
        Ok(clinic_service_json) => clinic_service_json,
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

    let data = clinic_service.into_inner();
    let is_valid = data.validate();

    match is_valid {
        Ok(_) => {
            match add_clinic_service(pool.get_ref(), &data, user_id).await {
                Ok(_) => return HttpResponse::Ok().json(ApiResponse::new("Successfully added")),
                Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(err)
        }
    }
}

pub async fn update(
    pool: web::Data<Pool<MySql>>, 
    id: web::Path<i32>,
    clinic_service: Result<web::Json<EditClinicService>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {
     // First handle JSON parsing errors
     let clinic_service = match clinic_service {
        Ok(clinic_service_json) => clinic_service_json,
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

    let clinic_service_data = clinic_service.into_inner();
    let is_valid = clinic_service_data.validate();

    match is_valid {
        Ok(_) => {
            match edit_clinic_service(pool.get_ref(), id.into_inner(), &clinic_service_data, user_id).await {
                Ok(Some(id)) => return HttpResponse::Ok().json(ApiResponse::new(format!("Successfully updated clinic_service_id: {}", id))),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("Clinic service not found")),
                Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(err)
        }
    }
}

pub async fn delete(
    pool: web::Data<Pool<MySql>>, 
    id: web::Path<i32>,
    req: HttpRequest
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user_id = claims.sub.parse::<i32>().unwrap();

    match delete_clinic_service(
        pool.get_ref(),
        id.into_inner(),
        user_id
    ).await {
        Ok(Some(deleted_id)) => HttpResponse::Ok().json(ApiResponse::new(format!("Successfully deleted clinic_service_id: {}", deleted_id))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("Clinic service not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
    }
}
