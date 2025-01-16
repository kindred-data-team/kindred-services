use serde::Serialize;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::sessions;

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub expires_at: NaiveDateTime,
    pub rbac_id: Uuid,
    pub user_id: i32
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub expires_at: NaiveDateTime, 
    pub created_at: NaiveDateTime, 
    pub updated_at: NaiveDateTime, 
    pub revoked: bool,
    pub rbac_id: Uuid,
    pub user_id: i32
}
