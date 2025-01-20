use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

impl ApiResponse {
    pub fn new(message: impl Into<String>) -> Self {
        ApiResponse {
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Meta {
    pub total: i64,
    pub page: usize,
    pub limit: usize,
    pub total_pages: usize
}