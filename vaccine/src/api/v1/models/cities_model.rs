use serde::{Deserialize, Serialize};
use chrono::Utc;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct City{
    pub id: u64,
    pub name: String,
    pub created_at: Option<chrono::DateTime<Utc>>,  
    pub updated_at: Option<chrono::DateTime<Utc>>, 
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCity {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String
}