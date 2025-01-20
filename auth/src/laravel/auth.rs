use anyhow::Error;

use crate::config::config::get_laravel_url;
use crate::models::{laravel::LaravelLoginResponse, users::UserLoginRequest};

pub async fn login_laravel(req: &UserLoginRequest) -> Result<LaravelLoginResponse, Error> {
    let client = reqwest::Client::new();
    let laravel_url = get_laravel_url();
    let resp = client.post(format!("{}/api/auth/login", laravel_url))
        .json(&req)
        .send()
        .await?;

    let json_resp: LaravelLoginResponse = resp.json().await?;
    Ok(json_resp)
}
