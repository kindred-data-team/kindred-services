use actix_web::{HttpRequest, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::users::NewUser;
use crate::models::response::ApiResponse;
use crate::repository::postgres::auth::{assign_default_role, insert_rbac_profile, insert_user, user_has_permission, validate_session};
use crate::config::config::get_default_role;

pub fn validate_expiration (expires_at: NaiveDateTime) -> Result<(), String>{
    let current_date:NaiveDateTime = Utc::now().naive_local();
    if expires_at < current_date {
        return Err("Session has expired".to_string());
    } else {
        return Ok(());
    }
}

fn extract_headers(req: &HttpRequest) -> Result<HashMap<String, String>, String> {
    let headers: Result<HashMap<_, _>, _> = req.headers()
        .iter()
        .map(|(name, value)| {
            value.to_str()
                .map(|v| (name.to_string(), v.to_string()))
                .map_err(|e| format!("Failed to parse header {}: {}", name, e))
        })
        .collect();

    headers
}

pub fn extract_session_id(req: &HttpRequest) -> Result<Uuid, HttpResponse> {
    // Get the headers
    let extract_headers_result = extract_headers(&req);
    if let Err(e) = extract_headers_result {
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)));
    } else if let Ok(headers) = extract_headers_result.clone() {
        if let Some(auth_value) = headers.get("authorization") {
            if auth_value.is_empty() {
                return Err(HttpResponse::Unauthorized().json(ApiResponse::new("Authorization header is empty.")));
            }
        } else {
            return Err(HttpResponse::Unauthorized().json(ApiResponse::new("No session_id found in the request headers.")));
        }
    }

    let headers = extract_headers_result.unwrap();

    let parse_uuid = Uuid::parse_str(&headers["authorization"]);
    if let Err(e) = parse_uuid {
        return Err(HttpResponse::Unauthorized().json(ApiResponse::new(&format!("Failed to parse Uuid: {}", e))));
    }
    let session_id = parse_uuid.unwrap();
    Ok(session_id)
}

pub fn request_validator (req: HttpRequest) -> Result<String, HttpResponse> {
    // Get the path called
    let path = req.path();

    // Get session_id from headers
    let extract_call = extract_session_id(&req);
    if let Err(e) = extract_call {
        return Err(e);
    }
    let session_id = extract_call.unwrap();

    let permission_check = user_has_permission(session_id, path);

    if let Err(e) = permission_check{
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)));
    } else if permission_check.unwrap() == false {
        return Err(HttpResponse::Unauthorized().json(ApiResponse::new("Invalid access!")));
    }

    match validate_session(session_id) {
        Ok(token) => Ok(token),
        Err(e) => Err(HttpResponse::Unauthorized().json(ApiResponse::new(&e)))
    }
}

pub fn matches_permission_path(permission_path: &str, requested_path: &str) -> bool {
    if permission_path.ends_with("/*") {
        // Wildcard match: check if the requested path starts with the prefix
        let prefix = &permission_path[..permission_path.len() - 1];
        requested_path.starts_with(prefix)
    } else if permission_path.contains("{}"){
        let regex_path = regex::escape(permission_path).replace(r"\{\}", r"\d+");
        let re = regex::Regex::new(&format!("^{}$", regex_path)).unwrap();
        re.is_match(requested_path)
    } else {
        permission_path == requested_path
    }
}

pub fn registration_process (req: NewUser, role: Option<i32>) -> Result<HttpResponse, HttpResponse> {
    let rbac_id = match insert_user(&req) {
        Ok(id) => id,
        Err(e) => return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)))
    };

    // Create RBAC profile
    if let Err(e) = insert_rbac_profile(rbac_id){
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)));
    }

    // Assign default role
    let default_role_id = role.unwrap_or(get_default_role());
    match assign_default_role(rbac_id, default_role_id){
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::new("User Registered!"))),
        Err(e) => Err(HttpResponse::BadRequest().json(ApiResponse::new(&e)))
    }
}
