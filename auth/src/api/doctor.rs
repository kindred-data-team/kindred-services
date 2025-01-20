use actix_web::{HttpResponse, Responder, HttpRequest};
use crate::helper::utils::request_validator;
use crate::laravel::doctor::get_all_doctors_laravel;
use crate::models::response::ApiResponse;

pub async fn get_all_doctors(req: HttpRequest) -> impl Responder {
    
    let validated_request = request_validator(req);
    if let Err(e) = validated_request {
        return e;
    }
    let token = validated_request.unwrap();

    match get_all_doctors_laravel(token).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e.to_string()))
    }
}