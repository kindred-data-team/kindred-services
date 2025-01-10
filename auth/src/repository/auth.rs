use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;
use diesel::prelude::*;

use crate::models::{auth::{NewSession, Session}, request::LoginRequest, users::NewUser};
use crate::db::db::establish_connection;
use crate::models::users::UserLoginRequest;
use crate::helper::helper::{generate_token, hash_password, verif_pass};


pub fn insert_session(req: &LoginRequest){
    let user_id = req.session_id;
    println!("session_id: {user_id}");
    let new_token = generate_token();
    let new_expires_at:NaiveDateTime = Utc::now().naive_local() + Duration::hours(1);
    let today = Utc::now().naive_local();
    println!("Current date: {today}");
    println!("Expires: {:?}", new_expires_at);

    use crate::schema::sessions::dsl::*;

    let connection = establish_connection();

    let new_session = NewSession {
        token: new_token,
        expires_at: new_expires_at
    };

    diesel::insert_into(sessions)
        .values(&new_session)
        .execute(&connection)
        .expect("Error saving new session");
}

pub fn fetch_sessions() -> Vec<Session>{
    println!("Getting sessions!");

    use crate::schema::sessions::dsl::*;

    let connection = establish_connection();

    let results = sessions
        .filter(revoked.eq(false))
        .load::<Session>(&connection)
        .expect("Error loading sessions");

    return results;
}

pub fn insert_rbac_profile() -> Result<Uuid, String>{
    use crate::schema::rbac_profiles::dsl as rbac_dsl;

    let connection = establish_connection();

    let user_rbac_id = diesel::insert_into(rbac_dsl::rbac_profiles)
        .default_values()
        .returning(rbac_dsl::id)
        .get_result::<Uuid>(&connection)
        .map_err(|e| format!("Failed to create RBAC profile: {}", e))?;

    Ok(user_rbac_id)
}

pub fn insert_user(req: &NewUser) -> Result<(), String> {
    println!("Registering User!");

    use crate::schema::users::dsl::*;
    use crate::schema::role_assignments::dsl as ra_dsl;

    let connection = establish_connection();

    let hashed_pass = hash_password(&req.password);

    let new_user = NewUser {
        password: hashed_pass.expect("Error in parsing password!"),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        rbac_id: req.rbac_id.clone(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection)
        .map_err(|e| format!("Failed to create new user: {}", e))?;

    // Step 4: Assign Default Role
    let default_role_id = 1; // Assuming 'user' role ID is 1
    diesel::insert_into(ra_dsl::role_assignments)
        .values((
            ra_dsl::rbac_id.eq(&req.rbac_id),
            ra_dsl::role_id.eq(default_role_id),
        ))
        .execute(&connection)
        .map_err(|e| format!("Failed to assign default role: {}", e))?;

    Ok(())
}


pub fn get_user_login(req: &UserLoginRequest) -> Result<(), String>{
    println!("Getting User!");

    use crate::schema::users::dsl::*;

    let connection = establish_connection();

    let result = users.filter(email.eq(&req.email))
        .select(password)
        .limit(1)
        .first::<Option<String>>(&connection);

     let pass = match result {
        Ok(Some(pass)) => pass,
        Ok(None) => return Err("No password found for the given email.".to_string()),
        Err(e) => return Err(e.to_string()),
    };

    match verif_pass(&req.password, pass){
        Ok(_) => Ok(()),
        Err(_) => Err("Incorrect password!".to_string())
    }
    
}