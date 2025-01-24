use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;
use diesel::prelude::*;

use crate::models::users::{NewUser, SessionCredentials, UserCredentials, UserLoginRequest};
use crate::models::auth::{NewSession, Session, SessionDetails};
use crate::utils::helper::{matches_permission_path, validate_expiration};
use crate::config::config::get_session_duration;
use crate::db::db::establish_connection;

pub fn fetch_sessions() -> Vec<Session>{
    use crate::schema::sessions::dsl::*;

    let mut connection = establish_connection();

    let results = sessions
        .filter(revoked.eq(false))
        .load::<Session>(&mut connection)
        .expect("Error loading sessions");

    return results;
}

pub fn insert_rbac_profile(rbac_id: Uuid) -> Result<(), String>{
    use crate::schema::rbac_profiles::dsl as rbac_dsl;

    let mut connection = establish_connection();


    diesel::insert_into(rbac_dsl::rbac_profiles)
        .values(rbac_dsl::id.eq(rbac_id))
        .execute(&mut connection)
        .map_err(|e| format!("Failed to create RBAC profile: {}", e))?;

    Ok(())
}

pub fn insert_user(req: &NewUser) -> Result<Uuid, String> {
    use crate::schema::auth_users::dsl as users_dsl;

    let mut connection = establish_connection();

    let new_user = NewUser {
        email: req.email.clone(),
    };

    let rbac_id = diesel::insert_into(users_dsl::auth_users)
        .values(&new_user)
        .returning(users_dsl::rbac_id)
        .get_result::<Uuid>(&mut connection)
        .map_err(|e| format!("Failed to create new user: {}", e))?;

    Ok(rbac_id)
}

pub fn assign_default_role(rbac_id: Uuid, default_role_id: i32) -> Result<(), String>{
    use crate::schema::role_assignments::dsl as ra_dsl;

    let mut connection = establish_connection();

    diesel::insert_into(ra_dsl::role_assignments)
        .values((
            ra_dsl::rbac_id.eq(rbac_id),
            ra_dsl::role_id.eq(default_role_id),
        ))
        .execute(&mut connection)
        .map_err(|e| format!("Failed to assign default role: {}", e))?;

    Ok(())
}

fn create_session(rbac_id: Uuid, auth_user_id: i32, access_token: &String) -> Result<Uuid, String> {
    use crate::schema::sessions::dsl as sesh_dsl;

    let mut connection = establish_connection();

    let default_duration = get_session_duration();
    let new_expires_at:NaiveDateTime = Utc::now().naive_local() + Duration::days(default_duration);

    let new_session = NewSession {
        expires_at: new_expires_at,
        rbac_id,
        auth_user_id,
        access_token: access_token.to_string()
    };

    let session_id = diesel::insert_into(sesh_dsl::sessions)
        .values(&new_session)
        .returning(sesh_dsl::id)
        .get_result::<Uuid>(&mut connection)
        .map_err(|e| format!("Failed to create login session: {}", e))?;

    Ok(session_id)
}

pub fn get_new_session(session_id: &Uuid, access_token: &String) -> Result<Uuid, String> {
    use crate::schema::sessions::dsl as sesh_dsl;

    let mut connection = establish_connection();

    let result = sesh_dsl::sessions.filter(sesh_dsl::id.eq(session_id))
        .select((sesh_dsl::auth_user_id, sesh_dsl::rbac_id))
        .limit(1)
        .first::<SessionCredentials>(&mut connection);

    // Handle the query result
    let credentials = match result {
        Ok(credentials) => credentials,
        Err(e) => return Err(e.to_string()),
    };

    match create_session(credentials.rbac_id, credentials.user_id, access_token) {
        Ok(session_id) => Ok(session_id),
        Err(e) => Err(e)
    }
}

pub fn get_user_login(req: &UserLoginRequest, access_token: &String) -> Result<Uuid, String> {
    use crate::schema::auth_users::dsl::*;

    let mut connection = establish_connection();

    // Fetch auth_user_id and rbac_id
    let result = auth_users.filter(email.eq(&req.email))
        .select((id, rbac_id))
        .limit(1)
        .first::<UserCredentials>(&mut connection);

    // Handle the query result
    let credentials: UserCredentials = match result {
        Ok(credentials) => credentials,
        Err(e) => return Err(e.to_string()),
    };

    match create_session(credentials.rbac_id, credentials.id, access_token) {
        Ok(session_id) => Ok(session_id),
        Err(e) => Err(e)
    }
}

pub fn auth_path_check(permission_path: String) -> Result<bool, String> {
    let mut connection = establish_connection();
    use crate::schema::permissions::dsl as p_dsl;

    let permissions: Vec<String> = p_dsl::permissions
        .filter(p_dsl::is_private.eq(false))
        .select(p_dsl::path)
        .load(&mut connection)
        .map_err(|e| format!("Failed to fetch permissions: {}", e))?;

    let auth_check = permissions
        .iter()
        .any(|path| matches_permission_path(path, &permission_path));

    Ok(auth_check)
}

pub fn user_has_permission(
    session_id: Uuid,
    permission_path: &str,
) -> Result<bool, String> {

    let mut connection = establish_connection();

    use crate::schema::sessions::dsl as sesh_dsl;
    use crate::schema::permissions::dsl as p_dsl;
    use crate::schema::role_assignments::dsl as ra_dsl;
    use crate::schema::role_permissions::dsl as rp_dsl;

    // Fetch RBAC ID from sessions
    let r_id: Uuid = sesh_dsl::sessions
        .filter(sesh_dsl::id.eq(session_id))
        .filter(sesh_dsl::revoked.eq(false))
        .select(sesh_dsl::rbac_id)
        .first(&mut connection)
        .map_err(|e| format!("Failed to fetch rbac_id from sessions: {}", e))?;

    // Check role-based permissions
    let role_permissions: Vec<String> = ra_dsl::role_assignments
        .inner_join(rp_dsl::role_permissions.on(ra_dsl::role_id.eq(rp_dsl::role_id)))
        .inner_join(p_dsl::permissions.on(rp_dsl::permission_id.eq(p_dsl::id)))
        .filter(ra_dsl::rbac_id.eq(r_id))
        .select(p_dsl::path)
        .load(&mut connection)
        .map_err(|e| format!("Failed to fetch role permissions: {}", e))?;

    let has_role_permission = role_permissions
        .iter()
        .any(|path| matches_permission_path(path, permission_path));

    Ok(has_role_permission)
}

pub fn validate_session(session_id: Uuid) -> Result<String, String> {
    let mut connection = establish_connection();

    use crate::schema::sessions::dsl as sesh_dsl;

    let session: SessionDetails = sesh_dsl::sessions
        .filter(sesh_dsl::id.eq(session_id))
        .filter(sesh_dsl::revoked.eq(false))
        .select((sesh_dsl::expires_at, sesh_dsl::access_token))
        .first(&mut connection)
        .map_err(|e| format!("Failed to fetch rbac_id from sessions: {}", e))?;

    match validate_expiration(session.expires_at){
        Ok(_) => Ok(session.access_token.unwrap()),
        Err(e) => Err(e)
    } 
}
