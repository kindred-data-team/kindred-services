use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{permissions, roles, role_permissions};

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "roles"]
pub struct NewRole {
    pub name: String
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "role_permissions"]
pub struct NewRolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "permissions"]
pub struct NewPermission {
    pub path: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub path: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ProfilePermission {
    pub rbac_id: Uuid,
    pub permission_id: i32
}

#[derive(Serialize, Deserialize)]

pub enum RBACResult {
    Permission(Permission),
    Role(Role),
    RolePermission(Vec<RolePermission>),
    ProfilePermission(Vec<ProfilePermission>),
    SingeRolePermission(RolePermission)
}