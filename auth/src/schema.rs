// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (id) {
        id -> Int4,
        path -> Text,
    }
}

diesel::table! {
    profile_denied_permissions (rbac_id, permission_id) {
        rbac_id -> Uuid,
        permission_id -> Int4,
    }
}

diesel::table! {
    profile_permissions (rbac_id, permission_id) {
        rbac_id -> Uuid,
        permission_id -> Int4,
    }
}

diesel::table! {
    rbac_profiles (id) {
        id -> Uuid,
    }
}

diesel::table! {
    role_assignments (rbac_id, role_id) {
        rbac_id -> Uuid,
        role_id -> Int4,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        revoked -> Bool,
        rbac_id -> Uuid,
        user_id -> Int4,
    }
}

diesel::table! {
    user_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        permission_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        is_regular -> Nullable<Bool>,
        is_doctor -> Nullable<Bool>,
        ops_team -> Nullable<Bool>,
        email_verified_at -> Nullable<Timestamp>,
        password -> Nullable<Varchar>,
        google_id -> Nullable<Varchar>,
        apple_id -> Nullable<Varchar>,
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        rbac_id -> Uuid,
    }
}

diesel::joinable!(profile_denied_permissions -> permissions (permission_id));
diesel::joinable!(profile_denied_permissions -> rbac_profiles (rbac_id));
diesel::joinable!(profile_permissions -> permissions (permission_id));
diesel::joinable!(profile_permissions -> rbac_profiles (rbac_id));
diesel::joinable!(role_assignments -> rbac_profiles (rbac_id));
diesel::joinable!(role_assignments -> roles (role_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(sessions -> rbac_profiles (rbac_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_permissions -> permissions (permission_id));
diesel::joinable!(user_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    profile_denied_permissions,
    profile_permissions,
    rbac_profiles,
    role_assignments,
    role_permissions,
    roles,
    sessions,
    user_permissions,
    users,
);