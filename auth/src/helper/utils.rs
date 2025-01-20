use actix_web::{HttpRequest, HttpResponse};
use anyhow::Result;
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{laravel::profile::get_user_profile, models::users::{NewUser, UserLoginRequest}, repository::auth::{user_has_permission, validate_session}};
use crate::models::response::ApiResponse;
use crate::repository::auth::{assign_default_role, insert_rbac_profile, insert_user};


pub fn hash_password(pass: &String) -> Result<String, argon2::password_hash::Error>{
    let password = pass.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}

pub fn verif_pass(password: &str, password_hash: String) -> Result<(), String>{
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(_) => Err("Incorrect password!".to_string()),
    }
}

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
pub fn request_validator (req: HttpRequest) -> Result<String, HttpResponse> {
    // Get the path called
    let path = req.path();

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

pub fn registration_process (req: NewUser) -> Result<HttpResponse, HttpResponse> {
    let rbac_id = match insert_user(&req) {
        Ok(id) => id,
        Err(e) => return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)))
    };

    // Create RBAC profile
    if let Err(e) = insert_rbac_profile(rbac_id){
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)));
    }

    // Assign default role
    let default_role_id = 2; // Default role 'regular'
    match assign_default_role(rbac_id, default_role_id){
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::new("User Registered!"))),
        Err(e) => Err(HttpResponse::BadRequest().json(ApiResponse::new(&e)))
    }
}

pub async fn sync_user (token: &String, login_request: &UserLoginRequest) -> Result<(), HttpResponse>{
    let user_profile = get_user_profile(token.to_string()).await;
    if let Err(_) = user_profile{
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch profile")));
    }
    let user_profile_details = user_profile.unwrap();
    if let Some(first_name) = user_profile_details.get("first_name") {
        if let Some(last_name) = user_profile_details.get("last_name") {
            let new_user = NewUser {
                first_name: first_name.to_string().replace("\"", ""),
                last_name: last_name.to_string().replace("\"", ""),
                email: login_request.email.clone(),
                password: login_request.password.clone(),
            };
            match registration_process(new_user) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e)
            }
        }
    }
    return Err(HttpResponse::InternalServerError().json(ApiResponse::new("Failed to sync user.")));
}

// pub fn check_resource_access(rbac_id: &str, resource_scope: &str) -> bool {
//     if let Some(profile) = self.users.get(rbac_id) {
//         if profile.denied_permissions.contains(resource_scope) {
//             return false;
//         }

//         if profile.direct_permissions.contains(resource_scope) {
//             return true;
//         }

//         for role_name in &profile.roles {
//             if let Some(role) = self.roles.get(role_name) {
//                 if role.permissions.contains(resource_scope) {
//                     return true;
//                 }

//                 for permission in &role.permissions {
//                     if permission.ends_with("*") && resource_scope.starts_with(permission.trim_end_matches('*')) {
//                         return true;
//                     }
//                 }
//             }
//         }
//     }
//     false
// }