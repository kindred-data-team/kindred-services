use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaravelLoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32
}
