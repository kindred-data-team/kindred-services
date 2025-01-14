use actix_web::{HttpRequest, HttpResponse};
use anyhow::Result;
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::repository::auth::{user_has_permission, validate_session};
use crate::models::response::ApiResponse;


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

pub fn request_validator (req: HttpRequest) -> Result<(), HttpResponse> {
    // Get the path called
    let path = req.path();

    // Get the headers
    let headers: HashMap<_, _> = req.headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let session_id = Uuid::parse_str(&headers["authorization"]).unwrap();


    let permission_check = user_has_permission(session_id, path);

    if let Err(e) = permission_check{
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new(&e)));
    } else if permission_check.unwrap() == false {
        return Err(HttpResponse::Unauthorized().json(ApiResponse::new("Invalid access!")));
    }

    match validate_session(session_id) {
        Ok(_) => Ok(()),
        Err(e) => Err(HttpResponse::Unauthorized().json(ApiResponse::new(&e)))
    }
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