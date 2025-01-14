use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

impl ApiResponse {
    pub fn new(message: &str) -> Self {
        ApiResponse {
            message: message.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    session_id: Uuid,
}

impl LoginResponse {
    pub fn new(session_id: Uuid) -> Self {
        LoginResponse {
            session_id,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResult {
    pub status: u32,
    pub message: String
}

impl ErrorResult {
    pub fn new(status: u32, message: String) -> Self {
        ErrorResult {
            status,
            message
        }
    }
}