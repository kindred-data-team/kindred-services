use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::helper::helper::request_validator;
use crate::models::rbac::RBACRequest;
use crate::models::response::ApiResponse;
use crate::repository::rbac::rbac_db;

pub async fn handle_rbac(req: HttpRequest, req_body: web::Json<RBACRequest>) -> impl Responder {
    if let Err(e) = request_validator(req) {
        return e;
    }

    match rbac_db(req_body.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::new(&e))
    }
}