use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::laravel::auth::login_laravel;
use crate::helper::helper::request_validator;
use crate::models::response::{ApiResponse, LoginResponse};
use crate::models::users::{NewUser, NewUserRequest, UserLoginRequest};
use crate::repository::auth::{assign_default_role, fetch_sessions, get_user_login, insert_rbac_profile, insert_user};
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

    let rbac_id = match insert_user(&new_user) {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::new(&e))
    };

    // Create RBAC profile
    if let Err(e) = insert_rbac_profile(rbac_id){
        return HttpResponse::InternalServerError().json(ApiResponse::new(&e));
    }

    // Assign default role
    let default_role_id = 1; // Default role 'user'
    match assign_default_role(rbac_id, default_role_id){
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("User Registered!")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
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

    let login_result = login_laravel(&request).await;
    if let Err(e) = login_result {
        return HttpResponse::BadRequest().json(e.to_string());
    }
    let response = login_result.unwrap();
    
    match get_user_login(&request, &response.access_token){
        Ok(session_id) => HttpResponse::Ok().json(LoginResponse::new(session_id, response.access_token, response.token_type, response.expires_in)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
    }
}