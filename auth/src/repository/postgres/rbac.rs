use anyhow::Result;
use uuid::Uuid;
use diesel::prelude::*;

use crate::models::rbac::{MyField, NewPermission, NewRole, NewRoleAssignment, NewRolePermission, Permission, RBACRequest, RBACResult, Role, RoleAssignment, RolePermission};
use crate::db::db::establish_connection;

pub fn get_permission(mut connection: PgConnection, id: i32) -> Result<Permission, String>{
    use crate::schema::permissions::dsl as p_dsl;

    let permission: Permission = p_dsl::permissions
        .filter(p_dsl::id.eq(id))
        .first(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(permission)
}

pub fn get_permissions(mut connection: PgConnection) -> Result<Vec<Permission>, String>{
    use crate::schema::permissions::dsl as p_dsl;

    let permission: Vec<Permission> = p_dsl::permissions
        .load(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(permission)
}

pub fn insert_permission(mut connection: PgConnection, req: NewPermission) -> Result<Permission, String> {
    use crate::schema::permissions::dsl as p_dsl;

    let new_permission = diesel::insert_into(p_dsl::permissions)
        .values(&req)
        .returning((p_dsl::id, p_dsl::path, p_dsl::is_private, p_dsl::created_at, p_dsl::updated_at))
        .get_result::<Permission>(&mut connection)
        .map_err(|e| format!("Failed to insert new permission: {}", e))?;

    Ok(new_permission)
}

pub fn delete_permission(mut connection: PgConnection, id: i32) -> Result<Permission, String>{
    use crate::schema::permissions::dsl as p_dsl;

    let permission: Permission = diesel::delete(p_dsl::permissions.filter(p_dsl::id.eq(id)))
        .returning((p_dsl::id, p_dsl::path, p_dsl::is_private, p_dsl::created_at, p_dsl::updated_at))
        .get_result::<Permission>(&mut connection)
        .map_err(|e| format!("Failed to delete permission: {}", e))?;

    Ok(permission)
}

pub fn get_role(mut connection: PgConnection, id: i32) -> Result<Role, String>{
    use crate::schema::roles::dsl as r_dsl;

    let role: Role = r_dsl::roles
        .filter(r_dsl::id.eq(id))
        .first(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role)
}

pub fn get_roles(mut connection: PgConnection) -> Result<Vec<Role>, String>{
    use crate::schema::roles::dsl as r_dsl;

    let role: Vec<Role> = r_dsl::roles
        .load(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role)
}

pub fn insert_role(mut connection: PgConnection, req: NewRole) -> Result<Role, String> {
    use crate::schema::roles::dsl as r_dsl;

    let new_role = diesel::insert_into(r_dsl::roles)
        .values(&req)
        .returning((r_dsl::id, r_dsl::name, r_dsl::created_at, r_dsl::updated_at))
        .get_result::<Role>(&mut connection)
        .map_err(|e| format!("Failed to insert new role: {}", e))?;

    Ok(new_role)
}

pub fn delete_role(mut connection: PgConnection, id: i32) -> Result<Role, String>{
    use crate::schema::roles::dsl as r_dsl;

    let role: Role = diesel::delete(r_dsl::roles.filter(r_dsl::id.eq(id)))
        .returning((r_dsl::id, r_dsl::name, r_dsl::created_at, r_dsl::updated_at))
        .get_result::<Role>(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role)
}

pub fn get_role_permissions(mut connection: PgConnection, id: i32) -> Result<Vec<RolePermission>, String>{
    use crate::schema::role_permissions::dsl as rp_dsl;

    let role_permission: Vec<RolePermission> = rp_dsl::role_permissions
        // .inner_join(rhs)
        .filter(rp_dsl::role_id.eq(id))
        .load::<RolePermission>(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role_permission)
}

pub fn insert_role_permission(mut connection: PgConnection, req: NewRolePermission) -> Result<RolePermission, String> {
    use crate::schema::role_permissions::dsl as rp_dsl;

    let new_role_permission = diesel::insert_into(rp_dsl::role_permissions)
        .values(&req)
        .returning((rp_dsl::role_id, rp_dsl::permission_id, rp_dsl::created_at, rp_dsl::updated_at))
        .get_result::<RolePermission>(&mut connection)
        .map_err(|e| format!("Failed to insert new role permission: {}", e))?;

    Ok(new_role_permission)
}

pub fn delete_role_permissions(mut connection: PgConnection, req: RolePermission) -> Result<RolePermission, String>{
    use crate::schema::role_permissions::dsl as rp_dsl;

    let role_permission: RolePermission = diesel::delete(rp_dsl::role_permissions.filter(rp_dsl::role_id.eq(req.role_id)).filter(rp_dsl::permission_id.eq(req.permission_id)))
        .returning((rp_dsl::role_id, rp_dsl::permission_id, rp_dsl::created_at, rp_dsl::updated_at))
        .get_result::<RolePermission>(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role_permission)
}

pub fn get_role_assignment(mut connection: PgConnection, id: Uuid) -> Result<RoleAssignment, String>{
    use crate::schema::role_assignments::dsl as ra_dsl;

    let role_assignment: RoleAssignment = ra_dsl::role_assignments
        .filter(ra_dsl::rbac_id.eq(id))
        .first::<RoleAssignment>(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role_assignment)
}

pub fn update_role_assignment(mut connection: PgConnection, req: NewRoleAssignment) -> Result<RoleAssignment, String> {
    use crate::schema::role_assignments::dsl as ra_dsl;

    let new_role_assignment = diesel::update(ra_dsl::role_assignments.filter(ra_dsl::rbac_id.eq(req.rbac_id)))
        .set(ra_dsl::role_id.eq(req.role_id))
        .returning((ra_dsl::rbac_id, ra_dsl::role_id, ra_dsl::created_at, ra_dsl::updated_at))
        .get_result::<RoleAssignment>(&mut connection)
        .map_err(|e| format!("Failed to insert new role permission: {}", e))?;

    Ok(new_role_assignment)
}

pub fn delete_role_assignment(mut connection: PgConnection, id: Uuid) -> Result<RoleAssignment, String>{
    use crate::schema::role_assignments::dsl as ra_dsl;

    let role_assignment: RoleAssignment = diesel::delete(ra_dsl::role_assignments.filter(ra_dsl::rbac_id.eq(id)))
        .returning((ra_dsl::rbac_id, ra_dsl::role_id, ra_dsl::created_at, ra_dsl::updated_at))
        .get_result::<RoleAssignment>(&mut connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role_assignment)
}

pub fn rbac_db(rbac_request: RBACRequest) -> Result<RBACResult, String> {
    let connection = establish_connection();

    match rbac_request.method.as_str(){
        "get-permission" => {
            if let Some(MyField::RBACPermission(request)) = rbac_request.request {
                match get_permission(connection, request.permission_id) {
                    Ok(result) => return Ok(RBACResult::Permission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-permissions" => {
            match get_permissions(connection) {
                Ok(result) => return Ok(RBACResult::Permissions(result)),
                Err(e) => return Err(e)
            }
        },
        "get-role" => {
            if let Some(MyField::RBACRole(request)) = rbac_request.request {
                match get_role(connection, request.role_id) {
                    Ok(result) => return Ok(RBACResult::Role(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-roles" => {
            match get_roles(connection) {
                Ok(result) => return Ok(RBACResult::Roles(result)),
                Err(e) => return Err(e)
            }
        },
        "get-role-permissions" => {
            if let Some(MyField::RBACRole(request)) = rbac_request.request {
                match get_role_permissions(connection, request.role_id) {
                    Ok(result) => return Ok(RBACResult::RolePermission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-role-assignment" => {
            if let Some(MyField::RBACId(request)) = rbac_request.request {
                match get_role_assignment(connection, request.rbac_id) {
                    Ok(result) => return Ok(RBACResult::RoleAssignment(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-permission" => {
            if let Some(MyField::RBACAddPermission(request)) = rbac_request.request {
                let req = NewPermission { path: request.path, is_private: Some(request.is_private).unwrap() };
                match insert_permission(connection, req) {
                    Ok(result) => return Ok(RBACResult::Permission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-role" => {
            if let Some(MyField::RBACAddRole(request)) = rbac_request.request {
                let req = NewRole { name: request.name };
                match insert_role(connection, req) {
                    Ok(result) => return Ok(RBACResult::Role(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-role-permission" => {
            if let Some(MyField::RBACAddRolePermission(request)) = rbac_request.request {
                let req = NewRolePermission { role_id: request.role_id, permission_id: request.permission_id };
                match insert_role_permission(connection, req) {
                    Ok(result) => return Ok(RBACResult::SingeRolePermission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "update-role-assignment" => {
            if let Some(MyField::RBACUpdateRoleAssignment(request)) = rbac_request.request {
                let req = NewRoleAssignment { rbac_id: request.rbac_id, role_id: request.role_id };
                match update_role_assignment(connection, req) {
                    Ok(result) => return Ok(RBACResult::RoleAssignment(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "delete-permission" => {
            if let Some(MyField::RBACPermission(request)) = rbac_request.request {
                match delete_permission(connection, request.permission_id) {
                    Ok(result) => return Ok(RBACResult::Permission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "delete-role" => {
            if let Some(MyField::RBACRole(request)) = rbac_request.request {
                match delete_role(connection, request.role_id) {
                    Ok(result) => return Ok(RBACResult::Role(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "delete-role-permissions" => {
            if let Some(MyField::RBACDeleteRolePermission(request)) = rbac_request.request {
                match delete_role_permissions(connection, request) {
                    Ok(result) => return Ok(RBACResult::SingeRolePermission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "delete-role-assignment" => {
            if let Some(MyField::RBACId(request)) = rbac_request.request {
                match delete_role_assignment(connection, request.rbac_id) {
                    Ok(result) => return Ok(RBACResult::RoleAssignment(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        &_ => {return Err("Invalid request".to_string())}
    }
    
}