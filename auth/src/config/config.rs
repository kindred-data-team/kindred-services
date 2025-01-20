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
