use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    token: String,
    expires_at: String,
}
