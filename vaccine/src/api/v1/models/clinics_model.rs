use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Clinic {
    pub id: i32,
    pub name: Option<String>,
    pub is_online: Option<i32>,
    pub plato_code: Option<String>,
    pub availability_time: Option<String>,
    pub is_archived: Option<i32>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}