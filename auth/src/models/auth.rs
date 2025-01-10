use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::schema::sessions;

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub token: String,
    pub expires_at: NaiveDateTime
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub token: String,           
    pub expires_at: NaiveDateTime, 
    pub created_at: NaiveDateTime, 
    pub updated_at: NaiveDateTime, 
    pub revoked: bool
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Token {
    pub id: Uuid,                
    pub token: String,           
    pub expires_at: NaiveDateTime, 
    pub created_at: NaiveDateTime, 
    pub updated_at: NaiveDateTime, 
    pub revoked: bool
}

impl Token {
    pub fn new(
        id: Uuid,                
        token: String,           
        expires_at: NaiveDateTime, 
        created_at: NaiveDateTime, 
        updated_at: NaiveDateTime, 
        revoked: bool,
    ) -> Self {
        Token {
            id,
            token,
            expires_at,
            created_at,
            updated_at,
            revoked,
        }
    }
}