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