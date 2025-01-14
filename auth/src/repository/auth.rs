use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;
use diesel::prelude::*;

use crate::{helper::helper::validate_expiration, models::{auth::{NewSession, Session}, rbac::{NewPermission, NewRole, NewRolePermission, Permission, ProfilePermission, RBACResult, Role, RolePermission}, request::RBACRequest, users::NewUser}};
use crate::db::db::establish_connection;
use crate::models::users::{UserLoginRequest, UserCredentials};
use crate::models::request::MyField;
use crate::helper::helper::{hash_password, verif_pass};

pub fn fetch_sessions() -> Vec<Session>{
    use crate::schema::sessions::dsl::*;

    let connection = establish_connection();

    let results = sessions
        .filter(revoked.eq(false))
        .load::<Session>(&connection)
        .expect("Error loading sessions");

    return results;
}

pub fn insert_rbac_profile(rbac_id: Uuid) -> Result<(), String>{
    use crate::schema::rbac_profiles::dsl as rbac_dsl;

    let connection = establish_connection();


    diesel::insert_into(rbac_dsl::rbac_profiles)
        .values(rbac_dsl::id.eq(rbac_id))
        .execute(&connection)
        .map_err(|e| format!("Failed to create RBAC profile: {}", e))?;

    Ok(())
}

pub fn insert_user(req: &NewUser) -> Result<Uuid, String> {
    use crate::schema::users::dsl as users_dsl;

    let connection = establish_connection();

    let hashed_pass = hash_password(&req.password);

    let new_user = NewUser {
        password: hashed_pass.expect("Error in parsing password!"),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
    };

    let rbac_id = diesel::insert_into(users_dsl::users)
        .values(&new_user)
        .returning(users_dsl::rbac_id)
        .get_result::<Uuid>(&connection)
        .map_err(|e| format!("Failed to create new user: {}", e))?;

    Ok(rbac_id)
}

pub fn assign_default_role(rbac_id: Uuid, default_role_id: i32) -> Result<(), String>{
    use crate::schema::role_assignments::dsl as ra_dsl;

    let connection = establish_connection();

    diesel::insert_into(ra_dsl::role_assignments)
        .values((
            ra_dsl::rbac_id.eq(rbac_id),
            ra_dsl::role_id.eq(default_role_id),
        ))
        .execute(&connection)
        .map_err(|e| format!("Failed to assign default role: {}", e))?;

    Ok(())
}

pub fn get_user_login(req: &UserLoginRequest) -> Result<Uuid, String> {
    use crate::schema::users::dsl::*;
    use crate::schema::sessions::dsl as sesh_dsl;

    let connection = establish_connection();

    // Fetch both password and rbac_id
    let result = users.filter(email.eq(&req.email))
        .select((id, password, rbac_id))
        .limit(1)
        .first::<UserCredentials>(&connection);

    // Handle the query result
    let credentials = match result {
        Ok(credentials) => credentials,
        Err(e) => return Err(e.to_string()),
    };

    // Verify the password
    if let Err(e) = verif_pass(&req.password, credentials.password.unwrap()){
        return Err(e);
    }

    let new_expires_at:NaiveDateTime = Utc::now().naive_local() + Duration::hours(1);

    let new_session = NewSession {
        expires_at: new_expires_at,
        rbac_id: credentials.rbac_id,
        user_id: credentials.id,
    };

    let session_id = diesel::insert_into(sesh_dsl::sessions)
        .values(&new_session)
        .returning(sesh_dsl::id)
        .get_result::<Uuid>(&connection)
        .map_err(|e| format!("Failed to create login session: {}", e))?;

    Ok(session_id)
}


pub fn user_has_permission(
    session_id: Uuid,
    permission_path: &str,
) -> Result<bool, String> {

    let connection = establish_connection();

    use crate::schema::sessions::dsl as sesh_dsl;
    use crate::schema::profile_permissions::dsl as pp_dsl;
    use crate::schema::permissions::dsl as p_dsl;
    use crate::schema::role_assignments::dsl as ra_dsl;
    use crate::schema::role_permissions::dsl as rp_dsl;

    // Fetch RBAC ID from sessions
    let r_id: Uuid = sesh_dsl::sessions
        .filter(sesh_dsl::id.eq(session_id))
        .filter(sesh_dsl::revoked.eq(false))
        .select(sesh_dsl::rbac_id)
        .first(&connection)
        .map_err(|e| format!("Failed to fetch rbac_id from sessions: {}", e))?;

    // Check user-specific permissions
    let has_user_permission: bool = pp_dsl::profile_permissions
        .inner_join(p_dsl::permissions.on(pp_dsl::permission_id.eq(p_dsl::id)))
        .filter(pp_dsl::rbac_id.eq(r_id))
        .filter(p_dsl::path.eq(permission_path))
        .select(p_dsl::id)
        .first::<i32>(&connection)
        .optional()
        .map(|opt| opt.is_some())
        .map_err(|e| format!("Failed to check user permissions: {}", e))?;



    if has_user_permission {
        return Ok(true);
    }

    // Check role-based permissions
    let has_role_permission: bool = ra_dsl::role_assignments
        .inner_join(rp_dsl::role_permissions.on(ra_dsl::role_id.eq(rp_dsl::role_id)))
        .inner_join(p_dsl::permissions.on(rp_dsl::permission_id.eq(p_dsl::id)))
        .filter(ra_dsl::rbac_id.eq(r_id))
        .filter(p_dsl::path.eq(permission_path))
        .select(p_dsl::id)
        .limit(1)
        .first::<i32>(&connection)
        .optional()
        .map(|opt| opt.is_some())
        .map_err(|e| format!("Failed to check role permissions: {}", e))?;

    Ok(has_role_permission)
}

pub fn validate_session(session_id: Uuid) -> Result<(), String> {
    let connection = establish_connection();

    use crate::schema::sessions::dsl as sesh_dsl;

    let expire_date: NaiveDateTime = sesh_dsl::sessions
        .filter(sesh_dsl::id.eq(session_id))
        .filter(sesh_dsl::revoked.eq(false))
        .select(sesh_dsl::expires_at)
        .first(&connection)
        .map_err(|e| format!("Failed to fetch rbac_id from sessions: {}", e))?;

    match validate_expiration(expire_date){
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    } 
}

pub fn get_permission(connection: PgConnection, id: i32) -> Result<Permission, String>{
    use crate::schema::permissions::dsl as p_dsl;

    let permission: Permission = p_dsl::permissions
        .filter(p_dsl::id.eq(id))
        .first(&connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(permission)
}

pub fn get_role(connection: PgConnection, id: i32) -> Result<Role, String>{
    use crate::schema::roles::dsl as r_dsl;

    let role: Role = r_dsl::roles
        .filter(r_dsl::id.eq(id))
        .first(&connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role)
}

pub fn get_role_permissions(connection: PgConnection, id: i32) -> Result<Vec<RolePermission>, String>{
    use crate::schema::role_permissions::dsl as rp_dsl;

    let role_permission: Vec<RolePermission> = rp_dsl::role_permissions
        .filter(rp_dsl::role_id.eq(id))
        .load::<RolePermission>(&connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(role_permission)
}

pub fn get_profile_permissions(connection: PgConnection, id: Uuid) -> Result<Vec<ProfilePermission>, String>{
    use crate::schema::profile_permissions::dsl as pp_dsl;

    let profile_permissions: Vec<ProfilePermission> = pp_dsl::profile_permissions
        .filter(pp_dsl::rbac_id.eq(id))
        .load::<ProfilePermission>(&connection)
        .map_err(|e| format!("Failed to fetch permission: {}", e))?;

    Ok(profile_permissions)
}

pub fn insert_permission(connection: PgConnection, req: NewPermission) -> Result<Permission, String> {
    use crate::schema::permissions::dsl as p_dsl;

    let new_permission = diesel::insert_into(p_dsl::permissions)
        .values(&req)
        .returning((p_dsl::id, p_dsl::path))
        .get_result::<Permission>(&connection)
        .map_err(|e| format!("Failed to insert new permission: {}", e))?;

    Ok(new_permission)
}

pub fn insert_role(connection: PgConnection, req: NewRole) -> Result<Role, String> {
    use crate::schema::roles::dsl as r_dsl;

    let new_role = diesel::insert_into(r_dsl::roles)
        .values(&req)
        .returning((r_dsl::id, r_dsl::name))
        .get_result::<Role>(&connection)
        .map_err(|e| format!("Failed to insert new role: {}", e))?;

    Ok(new_role)
}

pub fn insert_role_permission(connection: PgConnection, req: NewRolePermission) -> Result<RolePermission, String> {
    use crate::schema::role_permissions::dsl as rp_dsl;

    let new_role_permission = diesel::insert_into(rp_dsl::role_permissions)
        .values(&req)
        .returning((rp_dsl::role_id, rp_dsl::permission_id))
        .get_result::<RolePermission>(&connection)
        .map_err(|e| format!("Failed to insert new role permission: {}", e))?;

    Ok(new_role_permission)
}

pub fn rbac_db(rbac_request: RBACRequest) -> Result<RBACResult, String> {
    let connection = establish_connection();

    match rbac_request.method.as_str(){
        "get-permission" => {
            if let Some(MyField::RBACPermission(permission)) = rbac_request.request {
                match get_permission(connection, permission.permission_id) {
                    Ok(permission_result) => return Ok(RBACResult::Permission(permission_result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-role" => {
            if let Some(MyField::RBACRole(role)) = rbac_request.request {
                match get_role(connection, role.role_id) {
                    Ok(role_result) => return Ok(RBACResult::Role(role_result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-role-permissions" => {
            if let Some(MyField::RBACRole(role_permission)) = rbac_request.request {
                match get_role_permissions(connection, role_permission.role_id) {
                    Ok(role_result) => return Ok(RBACResult::RolePermission(role_result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "get-profile-permissions" => {
            if let Some(MyField::RBACId(profile_permission)) = rbac_request.request {
                match get_profile_permissions(connection, profile_permission.rbac_id) {
                    Ok(result) => return Ok(RBACResult::ProfilePermission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-permission" => {
            if let Some(MyField::RBACAddPermission(permission)) = rbac_request.request {
                let req = NewPermission { path: permission.path };
                match insert_permission(connection, req) {
                    Ok(permission_result) => return Ok(RBACResult::Permission(permission_result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-role" => {
            if let Some(MyField::RBACAddRole(permission)) = rbac_request.request {
                let req = NewRole { name: permission.name };
                match insert_role(connection, req) {
                    Ok(result) => return Ok(RBACResult::Role(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        "add-role-permission" => {
            if let Some(MyField::RBACAddRolePermission(permission)) = rbac_request.request {
                let req = NewRolePermission { role_id: permission.role_id, permission_id: permission.permission_id };
                match insert_role_permission(connection, req) {
                    Ok(result) => return Ok(RBACResult::SingeRolePermission(result)),
                    Err(e) => return Err(e)
                }
            } else {
                return Err("Invalid request".to_string())
            }
        },
        &_ => {return Err("Invalid request".to_string())}
    }
    
}