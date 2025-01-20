use std::env;

use jsonwebtoken::Algorithm;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_host() -> String {
    env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn get_port() -> String {
    env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string())
}

pub fn get_jwt_algorithm() -> Algorithm {
    let algo = env::var("JWT_ALGORITHM").expect("JWT_ALGORITHM must be set");

    match algo.as_str() {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        "RS256" => Algorithm::RS256,
        _ => panic!("Unsupported algorithm specified"),
    }
}

pub fn get_jwt_secret_key() -> String {
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set")
}