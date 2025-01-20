use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use crate::api::v1::repositories::cities_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::cities_model::{*};
use crate::api::v1::middleware::authentication::Claims;
use validator::Validate;
use serde_json::json;

pub async fn index(
    pool: web::Data<Pool<MySql>>,
    req: HttpRequest
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    println!("User ID (sub): {}", claims.sub);
    println!("JWT ID (jti): {}", claims.jti);

    match get_cities(pool.get_ref()).await {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch cities.")),
    }
}

pub async fn show(
    pool: web::Data<Pool<MySql>>, 
    city_id: web::Path<i32>
) -> impl Responder {
    match get_city_by_id(pool.get_ref(), city_id.into_inner()).await {
        Ok(city) => HttpResponse::Ok().json(city),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::new("Vaccine not found.")),
    }
}

pub async fn insert(
    pool: web::Data<Pool<MySql>>, 
    city: Result<web::Json<CreateCity>, actix_web::Error>,
) -> impl Responder {
    // First handle JSON parsing errors
    let city = match city {
        Ok(city_json) => city_json,
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

    let city_data = city.into_inner();
    let is_valid = city_data.validate();

    match is_valid {
        Ok(_) => {
            match add_city(pool.get_ref(), city_data).await {
                Ok(city) => HttpResponse::Ok().json(city),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
            }
        }
        Err(err) => {
            HttpResponse::BadRequest().json(err)
        }
    }
}

pub async fn delete(
    pool: web::Data<Pool<MySql>>, 
    id: web::Path<i32>
) -> impl Responder {
    match delete_city(
        pool.get_ref(),
        id.into_inner()
    ).await {
        Ok(Some(deleted_id)) => HttpResponse::Ok().json(ApiResponse::new(format!("Successfully deleted city_id {}", deleted_id))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("City not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
    }
}
