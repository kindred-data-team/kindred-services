use actix_web::{HttpResponse, Responder, HttpRequest};
use crate::helper::helper::request_validator;
use crate::laravel::profile::get_user_profile;
use crate::models::response::ApiResponse;

pub async fn get_profile(req: HttpRequest) -> impl Responder {
    
    let validated_request = request_validator(req);
    if let Err(e) = validated_request {
        return e;
    }
    let token = validated_request.unwrap();

    match get_user_profile(token).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e.to_string()))
    }
}