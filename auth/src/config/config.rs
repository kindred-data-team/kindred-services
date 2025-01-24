use std::env;
use dotenv::dotenv;

pub fn get_host() -> String {
    dotenv().ok();
    env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
}

pub fn get_port() -> String {
    dotenv().ok();
    env::var("PORT").unwrap_or_else(|_| "8080".to_string())
}

pub fn get_laravel_url() -> String {
    dotenv().ok();
    env::var("LARAVEL_URL").expect("Laravel URL must be set!")
}

pub fn get_default_role() -> i32 {
    dotenv().ok();
    let role_str = env::var("DEFAULT_ROLE").unwrap_or_else(|_| "2".to_string());
    let default_role = role_str.parse().unwrap_or_else(|_| 2);
    return default_role;
}

pub fn get_session_duration() -> i64 {
    dotenv().ok();
    let duration_str = env::var("SESSION_DURATION_DAYS").unwrap_or_else(|_| "14".to_string());
    let session_duration  = duration_str.parse().unwrap_or_else(|_| 14);
    return session_duration;
}