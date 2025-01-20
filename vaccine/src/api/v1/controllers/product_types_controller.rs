use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::{MySql, Pool};
use crate::api::v1::repositories::product_types_repository::{*};
use crate::api::v1::models::response_model::ApiResponse;
use crate::api::v1::models::product_types_model::{*};
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
    let page = opts.page.unwrap_or(1);

    match get_product_types(pool.get_ref(), page, limit).await {
        Ok(product_types) => HttpResponse::Ok().json(product_types),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch cities.")),
    }
}

pub async fn show(
    pool: web::Data<Pool<MySql>>, 
    product_type_id: web::Path<i32>
) -> impl Responder {
    match get_product_type_by_id(pool.get_ref(), product_type_id.into_inner()).await {
        Ok(product_type) => HttpResponse::Ok().json(product_type),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::new("Product type not found.")),
    }
}

pub async fn insert(
    pool: web::Data<Pool<MySql>>, 
    product_type: Result<web::Json<CreateProductType>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {

    // First handle JSON parsing errors
    let product_type = match product_type {
        Ok(product_type_json) => product_type_json,
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

    let product_type_data = product_type.into_inner();
    let is_valid = product_type_data.validate();

    match is_valid {
        Ok(_) => {
            match add_product_type(pool.get_ref(), &product_type_data, user_id).await {
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
    product_type: Result<web::Json<EditProductType>, actix_web::Error>,
    req: HttpRequest
) -> impl Responder {
     // First handle JSON parsing errors
     let product_type = match product_type {
        Ok(product_type_json) => product_type_json,
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

    let product_type_data = product_type.into_inner();
    let is_valid = product_type_data.validate();

    match is_valid {
        Ok(_) => {
            match edit_product_type(pool.get_ref(), id.into_inner(), &product_type_data, user_id).await {
                Ok(_) => return HttpResponse::Ok().json(ApiResponse::new("Successfully updated")),
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

    match delete_product_type(
        pool.get_ref(),
        id.into_inner(),
        user_id
    ).await {
        Ok(Some(deleted_id)) => HttpResponse::Ok().json(ApiResponse::new(format!("Successfully deleted product_type_id {}", deleted_id))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::new("Product type not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(format!("{}", e)))
    }
}
