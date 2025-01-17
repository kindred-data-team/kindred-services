use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;
use diesel::prelude::*;

use crate::helper::helper::validate_expiration;
use crate::db::db::establish_connection;
use crate::models::users::{UserLoginRequest, UserCredentials, NewUser};
use crate::models::auth::{NewSession, Session, SessionDetails};
use crate::helper::helper::{hash_password, verif_pass};

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
    use crate::schema::users::dsl as users_dsl;

    let mut connection = establish_connection();

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

pub fn get_user_login(req: &UserLoginRequest, access_token: &String) -> Result<Uuid, String> {
    use crate::schema::users::dsl::*;
    use crate::schema::sessions::dsl as sesh_dsl;

    let mut connection = establish_connection();

    // Fetch both password and rbac_id
    let result = users.filter(email.eq(&req.email))
        .select((id, password, rbac_id))
        .limit(1)
        .first::<UserCredentials>(&mut connection);

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
        access_token: access_token.to_string()
    };

    let session_id = diesel::insert_into(sesh_dsl::sessions)
        .values(&new_session)
        .returning(sesh_dsl::id)
        .get_result::<Uuid>(&mut connection)
        .map_err(|e| format!("Failed to create login session: {}", e))?;

    Ok(session_id)
}


pub fn user_has_permission(
    session_id: Uuid,
    permission_path: &str,
) -> Result<bool, String> {

    let mut connection = establish_connection();

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
        .first(&mut connection)
        .map_err(|e| format!("Failed to fetch rbac_id from sessions: {}", e))?;

    // Check user-specific permissions
    let has_user_permission: bool = pp_dsl::profile_permissions
        .inner_join(p_dsl::permissions.on(pp_dsl::permission_id.eq(p_dsl::id)))
        .filter(pp_dsl::rbac_id.eq(r_id))
        .filter(p_dsl::path.eq(permission_path))
        .select(p_dsl::id)
        .first::<i32>(&mut connection)
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
        .first::<i32>(&mut connection)
        .optional()
        .map(|opt| opt.is_some())
        .map_err(|e| format!("Failed to check role permissions: {}", e))?;

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
