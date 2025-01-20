use actix_web::HttpResponse;
use anyhow::Error;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::config::config::get_laravel_url;
use crate::helper::helper::registration_process;
use crate::models::response::ApiResponse;
use crate::models::users::{NewUser, UserLoginRequest};

pub async fn get_user_profile (token: String) -> Result<Value, Error>{
    let client = reqwest::Client::new();
    let laravel_url = get_laravel_url();
    let resp = client.get(format!("{}/api/me", laravel_url))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;    

    let json_resp: Value = resp.json().await?;
    Ok(json_resp)
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

// Sample if you want to access a field from the response
    // if let Some(arr) = json_resp.as_array(){
    //     if let Some(first) = arr.get(0) {
    //         if let Some(first_name) = first.get("first_name"){
    //             println!("first_name: {}", first_name);
    //         }
    //     }
    // }