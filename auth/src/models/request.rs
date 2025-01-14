use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    token: String,
    expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACPermission {
    pub permission_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACAddPermission {
    pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACRole {
    pub role_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACAddRole {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACId {
    pub rbac_id: Uuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACAddRolePermission {
    pub role_id: i32,
    pub permission_id: i32
}

#[derive(Debug, Serialize, Deserialize,)]
#[serde(tag = "type")]
pub enum MyField {
    RBACPermission(RBACPermission),
    RBACRole(RBACRole),
    RBACId(RBACId),
    RBACAddPermission(RBACAddPermission),
    RBACAddRole(RBACAddRole),
    RBACAddRolePermission(RBACAddRolePermission)
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RBACRequest {
    pub method: String,
    pub request: Option<MyField>,
}
