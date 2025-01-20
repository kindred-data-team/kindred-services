use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use crate::schema::users;

#[derive(Insertable, Deserialize, Serialize, Validate)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Confirm password does not match password"))]
    pub password_confirmation: String
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct UserLoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Queryable)]
pub struct UserCredentials {
    pub id: i32,
    pub password: Option<String>,
    pub rbac_id: Uuid,
}