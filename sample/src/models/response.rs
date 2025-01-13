use serde_derive::Serialize;

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
