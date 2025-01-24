use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::laravel::auth::login_laravel;
use crate::helper::helper::{registration_process, request_validator};
use crate::laravel::profile::sync_user;
use crate::models::response::{ApiResponse, LoginResponse};
use crate::models::users::{NewUser, NewUserRequest, UserLoginRequest};
use crate::repository::auth::{fetch_sessions, get_user_login};
use validator::Validate;

pub async fn get_session(req: HttpRequest) -> impl Responder {
    if let Err(e) = request_validator(req) {
        return e;
    }

    let result = fetch_sessions();

    HttpResponse::Ok().json(result)
}

pub async fn register_user(req: web::Json<NewUserRequest>) -> impl Responder {
    let request = req.into_inner();
    // Validate the input
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    let new_user = NewUser {
        first_name: request.first_name.clone(),
        last_name: request.last_name.clone(),
        email: request.email.clone(),
        password: request.password.clone()
    };

    match registration_process(new_user) {
        Ok(resp) => return resp,
        Err(e) => return  e
    }

    // Assign permission
    // match assign_permission(rbac_id, default_role_id){
    //     Ok(_) => HttpResponse::Ok().json(ApiResponse::new("User Registered!")),
    //     Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
    // }
    
}

pub async fn login_user(req: web::Json<UserLoginRequest>) -> impl Responder {
    let request = req.into_inner();

    // Validate the input
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    };

    // Login on laravel
    let login_result = login_laravel(&request).await;
    if let Err(e) = login_result {
        return HttpResponse::BadRequest().json(e.to_string());
    }
    let response = login_result.unwrap();

    // Login on rust auth-service
    let login_request = get_user_login(&request, &response.access_token);
    if let Err(e) = login_request.clone() {
        //Check if account exists in laravel and sync if existing
        if !response.access_token.is_empty() {
        match sync_user(&response.access_token, &request).await {
            Ok(_) => {
                match get_user_login(&request, &response.access_token){
                    Ok(session_id) => return HttpResponse::Ok().json(LoginResponse::new(session_id, response.access_token, response.token_type, response.expires_in)),
                    Err(e) => return HttpResponse::BadRequest().json(ApiResponse::new(&e))
                }
            },
            Err(e) => return e
        }
    } else {
        return HttpResponse::BadRequest().json(ApiResponse::new(&e));
    }
} else if let Ok(session_id) = login_request {
    return HttpResponse::Ok().json(LoginResponse::new(session_id, response.access_token, response.token_type, response.expires_in))
}
return HttpResponse::BadRequest().json(ApiResponse::new("Failed to login user."));


    // match get_user_login(&request, &response.access_token){
    //     Ok(session_id) => HttpResponse::Ok().json(LoginResponse::new(session_id, response.access_token, response.token_type, response.expires_in)),
    //     Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
    // }

    
}