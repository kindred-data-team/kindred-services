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
    pub session_id: Uuid,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32
}

impl LoginResponse {
    pub fn new(session_id: Uuid, access_token: String, token_type: String, expires_in: u32) -> Self {
        LoginResponse {
            session_id,
            access_token,
            token_type,
            expires_in
        }
    }
}
