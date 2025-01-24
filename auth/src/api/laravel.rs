use actix_web::http::StatusCode;
use serde_json::Value;
use actix_web::{HttpResponse, Responder, HttpRequest, web};

use crate::repository::postgres::auth::auth_path_check;
use crate::repository::laravel::handler::handle_laravel_request;
use crate::utils::helper::request_validator;
use crate::models::response::ApiResponse;

pub async fn default_handler(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();

    let path_check = auth_path_check(req.path().to_string());
    if let Err(e) = path_check {
        return HttpResponse::InternalServerError().json(ApiResponse::new(&e))
    } 
    let path_check_result = path_check.unwrap();

    if path_check_result == true {
        match handle_laravel_request(None, req, request_body).await {
            Ok(result) => {
                let json_result: Value = serde_json::from_str(&result).unwrap();
                return HttpResponse::Ok().json(json_result);
            },
            Err(e) => return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message))
        }
    } else {
        let validated_request = request_validator(req.clone());
        if let Err(e) = validated_request {
            return e;
        }
        let token = validated_request.unwrap();

        match handle_laravel_request(Some(token), req, request_body).await {
            Ok(result) => {
                let json_result: Value = serde_json::from_str(&result).unwrap();
                return HttpResponse::Ok().json(json_result);
            },
            Err(e) => return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message))
        }
    }
}