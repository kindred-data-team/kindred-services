use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaravelRegisterLoginRefreshResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordResponse {
    pub token: String,
}
