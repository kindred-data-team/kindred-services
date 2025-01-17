use anyhow::Error;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::config::config::get_laravel_url;

pub async fn get_all_doctors_laravel(token: String) -> Result<Value, Error>{
    let client = reqwest::Client::new();
    let laravel_url = get_laravel_url();
    let resp = client.get(format!("{}/api/doctors", laravel_url))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;    

    let json_resp: Value = resp.json().await?;
    Ok(json_resp)
    
    // Sample if you want to access a field from the response
    // if let Some(arr) = json_resp.as_array(){
    //     if let Some(first) = arr.get(0) {
    //         if let Some(first_name) = first.get("first_name"){
    //             println!("first_name: {}", first_name);
    //         }
    //     }
    // }
}