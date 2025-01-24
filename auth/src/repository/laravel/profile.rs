use actix_web::HttpResponse;
use anyhow::Error;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::models::response::ApiResponse;
use crate::models::users::{NewUser, UserLoginRequest};
use crate::config::config::{get_default_role, get_laravel_url};
use crate::utils::helper::registration_process;

async fn get_user_profile (token: String) -> Result<Value, Error>{
    let client = reqwest::Client::new();
    let laravel_url = get_laravel_url();

    let mut request_builder = client.request(reqwest::Method::GET, format!("{}/api/me", laravel_url));
    request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {}", token));

    let resp = request_builder
        .send()
        .await?;

    let json_resp: Value = resp.json().await?;
    Ok(json_resp)
}

pub async fn sync_user (token: &String, login_request: Option<&UserLoginRequest>) -> Result<String, HttpResponse>{
    let user_profile = get_user_profile(token.to_string()).await;
    if let Err(_) = user_profile{
        return Err(HttpResponse::InternalServerError().json(ApiResponse::new("Failed to fetch profile")));
    }
    let user_profile_value = user_profile.unwrap();

    let mut user_type = get_default_role();
    if let Some(is_doctor) = user_profile_value.clone().get("is_doctor") {
        if is_doctor == 1 {
            user_type = 3;
        }
    }
    if let Some(ops_team) = user_profile_value.clone().get("ops_team") {
        if ops_team == 1 {
            user_type = 4;
        }
    }

    let email = if let Some(login) = login_request {
        login.email.to_string()
    } else {
        match user_profile_value.get("email") {
            Some(profile_email) => profile_email.to_string(),
            None => return Err(HttpResponse::BadRequest().json(ApiResponse::new("Email not found"))),
        }
    };

    let new_user = NewUser {
        email: email.clone()
    };
    match registration_process(new_user, Some(user_type)) {
        Ok(_) => return Ok(email),
        Err(e) => return Err(e)
    }
}