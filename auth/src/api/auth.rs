use actix_web::http::StatusCode;
use serde_json::Value;
use validator::Validate;
use actix_web::{web, HttpResponse, Responder, HttpRequest};

use crate::repository::laravel::handler::handle_laravel_request;
use crate::repository::laravel::profile::sync_user;
use crate::repository::postgres::auth::{fetch_sessions, get_new_session, get_user_login};
use crate::utils::helper::{extract_session_id, registration_process, request_validator};
use crate::models::laravel::{LaravelRegisterLoginRefreshResponse, ResetPasswordResponse};
use crate::models::response::{ApiResponse, LoginResponse, ResetResponse};
use crate::models::users::{NewUser, NewUserRequest, ResetPasswordRequest, SocialLoginRequest, UserLoginRequest};

pub async fn get_session(req: HttpRequest) -> impl Responder {
    if let Err(e) = request_validator(req) {
        return e;
    }

    let result = fetch_sessions();

    HttpResponse::Ok().json(result)
}

//User registration
pub async fn register_user(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();
    let request: NewUserRequest = serde_json::from_str(&request_body.to_string()).unwrap();
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    let register_result = handle_laravel_request(None, req, request_body).await;
    if let Err(e) = register_result {
        return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message));
    }
    let response = register_result.unwrap();
    let json_resp: LaravelRegisterLoginRefreshResponse = serde_json::from_str(&response).unwrap();

    let new_user = NewUser {
        email: request.email.clone(),
    };

    if let Err(e) = registration_process(new_user, None) {
        return  e;
    }
    let login_request = UserLoginRequest {
        email: request.email
    };
    match get_user_login(&login_request, &json_resp.access_token){
        Ok(session_id) => return HttpResponse::Ok().json(LoginResponse::new(session_id, json_resp.access_token, json_resp.token_type, json_resp.expires_in)),
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::new(&e))
    }
}

//User login
pub async fn login_user(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();
    let request: UserLoginRequest = serde_json::from_str(&request_body.to_string()).unwrap();
    
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    // Login on laravel
    let login_result = handle_laravel_request(None, req, request_body).await;
    if let Err(e) = login_result {
        return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message));
    }
    let response = login_result.unwrap();
    let json_resp: LaravelRegisterLoginRefreshResponse = serde_json::from_str(&response).unwrap();

    // Login on rust auth-service
    let login_request = get_user_login(&request, &json_resp.access_token);
    if let Err(e) = login_request.clone() {
        //Check if account exists in laravel and sync if existing
        if !json_resp.access_token.is_empty() {
        match sync_user(&json_resp.access_token, Some(&request)).await {
            Ok(_) => {
                match get_user_login(&request, &json_resp.access_token){
                    Ok(session_id) => return HttpResponse::Ok().json(LoginResponse::new(session_id, json_resp.access_token, json_resp.token_type, json_resp.expires_in)),
                    Err(e) => return HttpResponse::BadRequest().json(ApiResponse::new(&e))
                }
            },
            Err(e) => return e
        }
        } else {
            return HttpResponse::BadRequest().json(ApiResponse::new(&e));
        }
    } else if let Ok(session_id) = login_request {
        return HttpResponse::Ok().json(LoginResponse::new(session_id, json_resp.access_token, json_resp.token_type, json_resp.expires_in))
    }
return HttpResponse::BadRequest().json(ApiResponse::new("Failed to login user."));
    
}

//Refresh user token
pub async fn refresh_token(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let validated_request = request_validator(req.clone());
    if let Err(e) = validated_request {
        return e;
    }
    let old_token = validated_request.unwrap();

    let extract_call = extract_session_id(&req);
    if let Err(e) = extract_call {
        return e;
    }
    let session_id = extract_call.unwrap();

    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();

    // Refresh token on laravel
    let refresh_result = handle_laravel_request(Some(old_token), req, request_body).await;
    if let Err(e) = refresh_result {
        return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message));
    }
    let response = refresh_result.unwrap();
    let json_resp: LaravelRegisterLoginRefreshResponse = serde_json::from_str(&response).unwrap();

    // Refresh token on rust auth-service
    match get_new_session(&session_id, &json_resp.access_token) {
        Ok(session_id) => HttpResponse::Ok().json(LoginResponse::new(session_id, json_resp.access_token, json_resp.token_type, json_resp.expires_in)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(&e))
    }

}

//User login using social provider
pub async fn social_login(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();
    let request: SocialLoginRequest = serde_json::from_str(&request_body.to_string()).unwrap();

    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    // Login on laravel
    let login_result = handle_laravel_request(None, req, request_body).await;
    if let Err(e) = login_result {
        return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message));
    }
    let response = login_result.unwrap();
    let json_resp: LaravelRegisterLoginRefreshResponse = serde_json::from_str(&response).unwrap();

    // Login on rust auth-service
    match sync_user(&json_resp.access_token, None).await {
        Ok(email) => {
            match get_user_login(&UserLoginRequest{email}, &json_resp.access_token){
                Ok(session_id) => return HttpResponse::Ok().json(LoginResponse::new(session_id, json_resp.access_token, json_resp.token_type, json_resp.expires_in)),
                Err(e) => return HttpResponse::BadRequest().json(ApiResponse::new(&e))
            }
        },
        Err(e) => return e
    }
    
}

//Reset password
pub async fn reset_password(req: HttpRequest, body: Option<web::Json<Value>>) -> impl Responder {
    let request_body = body.map(|b| b.into_inner()).unwrap_or_default();
    let request: ResetPasswordRequest = serde_json::from_str(&request_body.to_string()).unwrap();

    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    // Reset password on laravel
    let reset_pass_result = handle_laravel_request(None, req, request_body).await;
    if let Err(e) = reset_pass_result {
        return HttpResponse::build(StatusCode::from_u16(e.status).unwrap()).json(ApiResponse::new(&e.message));
    }
    let response = reset_pass_result.unwrap();
    let json_resp: ResetPasswordResponse = serde_json::from_str(&response).unwrap();

    let login_request = UserLoginRequest {
        email: request.email
    };

    // Get session from auth-service
    match get_user_login(&login_request, &json_resp.token){
        Ok(session_id) => return HttpResponse::Ok().json(ResetResponse::new(session_id, json_resp.token)),
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::new(&e))
    }
    
}