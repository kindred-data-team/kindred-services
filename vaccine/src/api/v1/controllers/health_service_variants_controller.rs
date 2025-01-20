use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use crate::api::v1::repositories::health_service_variants_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::health_service_variants_model::{*};
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

    match get_health_service_variants(pool.get_ref(), page, limit, service_id).await {
        Ok(health_services) => HttpResponse::Ok().json(health_services),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch cities.")),
    }
}

pub async fn show(
    pool: web::Data<Pool<MySql>>, 
    health_service_variant_id: web::Path<i32>
) -> impl Responder {
    match get_health_service_variant_by_id(pool.get_ref(), health_service_variant_id.into_inner()).await {
        Ok(health_service) => HttpResponse::Ok().json(health_service),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::new("Vaccine not found.")),
    }
}

pub async fn insert(
    pool: web::Data<Pool<MySql>>, 
    health_service_variant: Result<web::Json<CreateHealthServiceVariant>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {

    // First handle JSON parsing errors
    let health_service_variant = match health_service_variant {
        Ok(health_service_variant_json) => health_service_variant_json,
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

    let data = health_service_variant.into_inner();
    let is_valid = data.validate();

    match is_valid {
        Ok(_) => {
            match add_health_service_variant(pool.get_ref(), &data, user_id).await {
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
    health_service: Result<web::Json<EditHealthServiceVariant>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {
     // First handle JSON parsing errors
     let health_service = match health_service {
        Ok(health_service_json) => health_service_json,
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

    let health_service_data = health_service.into_inner();
    let is_valid = health_service_data.validate();

    match is_valid {
        Ok(_) => {
            match edit_health_service_variant(pool.get_ref(), id.into_inner(), &health_service_data, user_id).await {
                Ok(Some(id)) => return HttpResponse::Ok().json(ApiResponse::new(format!("Successfully updated health_service_variant_id: {}", id))),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("Health service variant not found")),
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

    match delete_health_service_variant(
        pool.get_ref(),
        id.into_inner(),
        user_id
    ).await {
        Ok(Some(deleted_id)) => HttpResponse::Ok().json(ApiResponse::new(format!("Successfully deleted health_service_variant_id: {}", deleted_id))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("Health service variant not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
    }
}
