use actix_web::{web, HttpResponse, Responder};
use crate::models::response::ApiResponse;
use crate::models::request::LoginRequest;
use crate::models::users::{NewUser, NewUserRequest, UserLoginRequest};
use crate::repository::auth::{insert_session, fetch_sessions, insert_user, get_user_login, insert_rbac_profile};
use validator::Validate;

pub async fn create_session(req: web::Json<LoginRequest>) -> impl Responder {
    insert_session(&req);
    HttpResponse::Ok().json(ApiResponse::new("Success!"))
}

pub async fn get_session() -> impl Responder {
    let result = fetch_sessions();
    for item in &result{
        println!("{:?}", item)
    }
    HttpResponse::Ok().json(result)
}

pub async fn register_user(req: web::Json<NewUserRequest>) -> impl Responder {
    let request = req.into_inner();
    // Validate the input
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    }

    let rbac_id = insert_rbac_profile().unwrap();

    let new_user = NewUser {
        first_name: request.first_name.clone(),
        last_name: request.last_name.clone(),
        email: request.email.clone(),
        password: request.password.clone(),
        rbac_id
    };
    match insert_user(&new_user){
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("User Registered!")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
    }
    
}

pub async fn login_user(req: web::Json<UserLoginRequest>) -> impl Responder {
    let request = req.into_inner();
    // Validate the input
    if let Err(errors) = request.validate() {
        return HttpResponse::BadRequest().json(errors.into_errors());
    };
    match get_user_login(&request){
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("Login success!")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::new(&e))
    }
}