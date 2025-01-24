use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{permissions, roles, role_permissions, role_assignments};

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "roles"]
pub struct NewRole {
    pub name: String
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "role_permissions"]
pub struct NewRolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "role_assignments"]
pub struct NewRoleAssignment {
    pub rbac_id: Uuid,
    pub role_id: i32
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "permissions"]
pub struct NewPermission {
    pub path: String,
    pub is_private: Option<bool>
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub path: String,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RoleAssignment {
    pub rbac_id: Uuid,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ProfilePermission {
    pub rbac_id: Uuid,
    pub permission_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]

pub enum RBACResult {
    Permission(Permission),
    Permissions(Vec<Permission>),
    Role(Role),
    Roles(Vec<Role>),
    RolePermission(Vec<RolePermission>),
    ProfilePermission(Vec<ProfilePermission>),
    SingeRolePermission(RolePermission),
    RoleAssignment(RoleAssignment),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACPermission {
    pub permission_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACAddPermission {
    pub path: String,
    pub is_private: Option<bool>
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

#[derive(Debug, Serialize, Deserialize,)]
#[serde(tag = "type")]
pub enum MyField {
    RBACPermission(RBACPermission),
    RBACRole(RBACRole),
    RBACId(RBACId),
    RBACAddPermission(RBACAddPermission),
    RBACAddRole(RBACAddRole),
    RBACAddRolePermission(NewRolePermission),
    RBACUpdateRoleAssignment(NewRoleAssignment),
    RBACDeleteRolePermission(RolePermission)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBACRequest {
    pub method: String,
    pub request: Option<MyField>,
}
