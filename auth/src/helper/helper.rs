use anyhow::Result;
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};


pub fn hash_password(pass: &String) -> Result<String, argon2::password_hash::Error>{
    println!("password: {:?}", pass);
    let password = pass.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    println!("hashed: {:?}", password_hash);

    Ok(password_hash)
}

pub fn verif_pass(password: &str, password_hash: String) -> Result<(), String>{
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(_) => Err("Incorrect password!".to_string()),
    }
}

// pub fn check_resource_access(rbac_id: &str, resource_scope: &str) -> bool {
//     if let Some(profile) = self.users.get(rbac_id) {
//         if profile.denied_permissions.contains(resource_scope) {
//             return false;
//         }

//         if profile.direct_permissions.contains(resource_scope) {
//             return true;
//         }

//         for role_name in &profile.roles {
//             if let Some(role) = self.roles.get(role_name) {
//                 if role.permissions.contains(resource_scope) {
//                     return true;
//                 }

//                 for permission in &role.permissions {
//                     if permission.ends_with("*") && resource_scope.starts_with(permission.trim_end_matches('*')) {
//                         return true;
//                     }
//                 }
//             }
//         }
//     }
//     false
// }