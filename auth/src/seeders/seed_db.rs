use diesel::prelude::*;

use crate::models::rbac::{NewPermission, NewRole, NewRolePermission};
use crate::db::db::establish_connection;

fn seed_roles() {
    let mut connection = establish_connection();

    use crate::schema::roles::dsl as r_dsl;

    let new_roles = vec![
        NewRole { name: "admin".to_string() },
        NewRole { name: "regular".to_string() },
        NewRole { name: "doctor".to_string() },
        NewRole { name: "ops".to_string() },
    ];

    diesel::insert_into(r_dsl::roles)
        .values(&new_roles)
        .execute(&mut connection)
        .expect("Failed to seed roles.");

    println!("Seeded {} roles", new_roles.len());
}

fn seed_permissions() {
    let mut connection = establish_connection();

    use crate::schema::permissions::dsl as p_dsl;

    let permission_list = vec![
        ("/api/*", true),
        ("/api/auth/login", false),
        ("/api/auth/register", false),
        ("/api/social-auth/google/login", false),
        ("/api/social-auth/apple/login", false),
        ("/api/auth/forgot-password", false),
        ("/api/auth/reset-password", false),
        ("/api/auth/refresh", true),
        ("/api/firebase-tokens", false),
        ("/api/me", true),
        ("/api/me/notes", true),
        ("/api/me/approve-consent", true),
        ("/api/me/app-rating", true),
        ("/api/invoices", true),
        ("/api/me/payment-methods", true),
        ("/api/consultations/upcoming", true),
        ("/api/consultations/history", true),
        ("/api/consultations/{}/info", true),
        ("/api/consultations/{}/promotion/{}/calculate", true),
        ("/api/consultations/{}/pay", true),
        ("/api/consultations", true),
        ("/api/consultations/{}", true),
        ("/api/consultations/{}/cancel", true),
        ("/api/consultations/{}/reschedule", true),
        ("/api/consultations/for-today", true),
        ("/api/consultations/for-today-paginated", true),
        ("/api/consultations/{}/prescription", true),
        ("/api/consultations/{}/review", true),
        ("/api/consultations/{}/call-token", true),
        ("/api/consultations/{}/end-call", true),
        ("/api/consultations/need-followup", true),
        ("/api/services/all", true),
        ("/api/services", true),
        ("/api/services/{}/reviews", true),
        ("/api/time-slots", true),
        ("/api/cities", true),
        ("/api/notifications", true),
        ("/api/notifications/read/{}", true),
        ("/api/health-issus", true),
        ("/api/me/health-info", true),
        ("/api/medical-concern-groups", true),
        ("/api/user-cards", true),
        ("/api/join/videochat", true),
        ("/api/upload/signed-url", true),
        ("/broadcasting/auth", false),
        ("/api/rooms", true),
        ("/api/rooms/{}", true),
        ("/api/rooms/{}/join", true),
        ("/api/rooms/{}/messages", true),
        ("/api/rooms/ops-team", true),
        ("/api/rooms/ops-team/messages", true),
        ("/api/files", true),
        ("/api/user-direct-debit/initialize-account-linking", true),
        ("/api/user-direct-debit/validate-account-linking-token", true),
        ("/api/services/{}/doctors", true),
        ("/api/doctors", true),
        ("/api/clinics", true),
        ("/api/services/{}/clinics", true),
        ("/api/commonly-booked-services", true),
        ("/api/popular-doctors", true),
        ("/api/services/{}/time-slots", true),
    ];

    for (path, is_private) in permission_list.clone() {
        let new_path = NewPermission {
            path: path.to_string(),
            is_private: Some(is_private),
        };

        diesel::insert_into(p_dsl::permissions)
            .values(&new_path)
            .execute(&mut connection)
            .expect("Failed to seed permissions.");
    }

    println!("Seeded {} permissions", permission_list.len());
}

fn seed_admin_role_permission() {
    let mut connection = establish_connection();

    use crate::schema::role_permissions::dsl as rp_dsl;

    let new_role_permission = NewRolePermission {
        role_id: 1,
        permission_id: 1
    };

    diesel::insert_into(rp_dsl::role_permissions)
        .values(&new_role_permission)
        .execute(&mut connection)
        .expect("Failed to seed admin role permission.");

    println!("Seeded 1 admin role permission");
}

fn seed_regular_role_permissions() {
    let mut connection = establish_connection();

    use crate::schema::{permissions::dsl as p_dsl, role_permissions::dsl as rp_dsl};

    // List of paths to match
    let paths = vec![
        "/api/auth/login",
        "/api/auth/register",
        "/api/social-auth/google/login",
        "/api/social-auth/apple/login",
        "/api/auth/forgot-password",
        "/api/auth/reset-password",
        "/api/auth/refresh",
        "/api/firebase-tokens",
        "/api/me",
        "/api/me/notes",
        "/api/me/approve-consent",
        "/api/me/app-rating",
        "/api/invoices",
        "/api/me/payment-methods",
        "/api/consultations/upcoming",
        "/api/consultations/history",
        "/api/consultations/{}/info",
        "/api/consultations/{}/promotion/{}/calculate",
        "/api/consultations/{}/pay",
        "/api/consultations",
        "/api/consultations/{}",
        "/api/consultations/{}/cancel",
        "/api/consultations/{}/reschedule",
        "/api/consultations/{}/prescription",
        "/api/consultations/{}/review",
        "/api/consultations/{}/call-token",
        "/api/consultations/{}/end-call",
        "/api/consultations/need-followup",
        "/api/services/all",
        "/api/services",
        "/api/services/{}/reviews",
        "/api/cities",
        "/api/notifications",
        "/api/notifications/read/{}",
        "/api/health-issus",
        "/api/me/health-info",
        "/api/medical-concern-groups",
        "/api/user-cards",
        "/api/join/videochat",
        "/api/upload/signed-url",
        "/broadcasting/auth",
        "/api/rooms",
        "/api/rooms/{}",
        "/api/rooms/{}/join",
        "/api/rooms/{}/messages",
        "/api/rooms/ops-team",
        "/api/rooms/ops-team/messages",
        "/api/files",
        "/api/user-direct-debit/initialize-account-linking",
        "/api/user-direct-debit/validate-account-linking-token",
        "/api/services/{}/doctors",
        "/api/doctors",
        "/api/clinics",
        "/api/services/{}/clinics",
        "/api/commonly-booked-services",
        "/api/popular-doctors",
        "/api/services/{}/time-slots",
    ];

    // Query the permissions table to get the IDs of the matching paths
    let permission_ids: Vec<i32> = p_dsl::permissions
        .filter(p_dsl::path.eq_any(paths))
        .select(p_dsl::id)
        .load(&mut connection)
        .expect("Failed to query permissions.");

    // Insert the role-permission mappings into the role_permissions table
    let role_id = 2; // The role ID to associate with the permissions
    let new_role_permissions: Vec<NewRolePermission> = permission_ids
        .into_iter()
        .map(|permission_id| NewRolePermission {
            role_id,
            permission_id,
        })
        .collect();

    diesel::insert_into(rp_dsl::role_permissions)
        .values(&new_role_permissions)
        .execute(&mut connection)
        .expect("Failed to seed regular role permissions.");

    println!("Seeded {} regular role permissions", new_role_permissions.len());
}

fn seed_doctor_role_permissions() {
    let mut connection = establish_connection();

    use crate::schema::{permissions::dsl as p_dsl, role_permissions::dsl as rp_dsl};

    // List of paths to match
    let paths = vec![
        "/api/auth/login",
        "/api/social-auth/google/login",
        "/api/social-auth/apple/login",
        "/api/auth/forgot-password",
        "/api/auth/reset-password",
        "/api/auth/refresh",
        "/api/firebase-tokens",
        "/api/me",
        "/api/me/notes",
        "/api/consultations/upcoming",
        "/api/consultations/history",
        "/api/consultations/{}",
        "/api/consultations/{}/cancel",
        "/api/consultations/{}/reschedule",
        "/api/consultations/for-today",
        "/api/consultations/{}/prescription",
        "/api/consultations/{}/call-token",
        "/api/consultations/{}/end-call",
        "/api/notifications",
        "/api/notifications/read/{}",
        "/api/upload/signed-url",
        "/broadcasting/auth",
        "/api/rooms",
        "/api/rooms/{}",
        "/api/rooms/{}/join",
        "/api/rooms/{}/messages",
        "/api/rooms/ops-team",
        "/api/rooms/ops-team/messages",
        "/api/time-slots",
        "/api/commonly-booked-services",
    ];

    // Query the permissions table to get the IDs of the matching paths
    let permission_ids: Vec<i32> = p_dsl::permissions
        .filter(p_dsl::path.eq_any(paths))
        .select(p_dsl::id)
        .load(&mut connection)
        .expect("Failed to query permissions.");

    // Insert the role-permission mappings into the role_permissions table
    let role_id = 3; // The role ID to associate with the permissions
    let new_role_permissions: Vec<NewRolePermission> = permission_ids
        .into_iter()
        .map(|permission_id| NewRolePermission {
            role_id,
            permission_id,
        })
        .collect();

    diesel::insert_into(rp_dsl::role_permissions)
        .values(&new_role_permissions)
        .execute(&mut connection)
        .expect("Failed to seed doctor role permissions.");

    println!("Seeded {} doctor role permissions", new_role_permissions.len());
}

fn seed_ops_role_permissions() {
    let mut connection = establish_connection();

    use crate::schema::{permissions::dsl as p_dsl, role_permissions::dsl as rp_dsl};

    // List of paths to match
    let paths = vec![
        "/api/auth/login",
        "/api/social-auth/google/login",
        "/api/social-auth/apple/login",
        "/api/auth/forgot-password",
        "/api/auth/reset-password",
        "/api/auth/refresh",
        "/api/firebase-tokens",
        "/api/me",
        "/api/me/notes",
        "/api/consultations/{}",
        "/api/consultations/for-today-paginated",
        "/api/notifications",
        "/api/notifications/read/{}",
        "/api/upload/signed-url",
        "/broadcasting/auth",
        "/api/rooms",
        "/api/rooms/{}",
        "/api/rooms/{}/join",
        "/api/rooms/{}/messages",
        "/api/rooms/ops-team",
        "/api/rooms/ops-team/messages",
    ];

    // Query the permissions table to get the IDs of the matching paths
    let permission_ids: Vec<i32> = p_dsl::permissions
        .filter(p_dsl::path.eq_any(paths))
        .select(p_dsl::id)
        .load(&mut connection)
        .expect("Failed to query permissions.");

    // Insert the role-permission mappings into the role_permissions table
    let role_id = 4; // The role ID to associate with the permissions
    let new_role_permissions: Vec<NewRolePermission> = permission_ids
        .into_iter()
        .map(|permission_id| NewRolePermission {
            role_id,
            permission_id,
        })
        .collect();

    diesel::insert_into(rp_dsl::role_permissions)
        .values(&new_role_permissions)
        .execute(&mut connection)
        .expect("Failed to seed ops role permissions.");

    println!("Seeded {} ops role permissions", new_role_permissions.len());
}

pub fn seeder () {
    seed_roles();
    seed_permissions();
    seed_admin_role_permission();
    seed_regular_role_permissions();
    seed_doctor_role_permissions();
    seed_ops_role_permissions();
}