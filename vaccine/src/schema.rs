// @generated automatically by Diesel CLI.

diesel::table! {
    clinic_service_time_slots (id) {
        id -> Integer,
        clinic_id -> Integer,
        product_type_id -> Integer,
        date -> Nullable<Date>,
        start_time -> Nullable<Time>,
        end_time -> Nullable<Time>,
        is_available -> Nullable<Bool>,
        is_virtual -> Nullable<Bool>,
        number_of_slots -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    clinic_services (id) {
        id -> Integer,
        clinic_id -> Integer,
        service_id -> Integer,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        plato_code -> Varchar,
    }
}

diesel::table! {
    clinics (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        plato_code -> Varchar,
        is_online -> Nullable<Bool>,
        #[max_length = 255]
        availability_time -> Varchar,
        is_archived -> Nullable<Bool>,
        archived_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    health_service_variants (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        service_id -> Integer,
        number_of_dose -> Nullable<Integer>,
        price -> Nullable<Integer>,
        #[max_length = 255]
        shopify_id -> Nullable<Varchar>,
        #[max_length = 255]
        shopify_sku -> Nullable<Varchar>,
        #[max_length = 255]
        shopify_variant_id -> Nullable<Varchar>,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    health_services (id) {
        id -> Integer,
        product_type_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        details -> Nullable<Text>,
        #[max_length = 255]
        for_whom -> Nullable<Varchar>,
        #[max_length = 255]
        shopify_id -> Nullable<Varchar>,
        #[max_length = 255]
        shopify_sku -> Nullable<Varchar>,
        #[max_length = 255]
        shopify_variant_id -> Nullable<Varchar>,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_types (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    service_appointments (id) {
        id -> Integer,
        patient_id -> Integer,
        clinic_id -> Integer,
        service_variant_id -> Integer,
        service_time_slot_id -> Integer,
        dosage_number -> Nullable<Integer>,
        doctor_id -> Nullable<Integer>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        created_by -> Nullable<Integer>,
        updated_by -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(clinic_service_time_slots -> clinics (clinic_id));
diesel::joinable!(clinic_service_time_slots -> product_types (product_type_id));
diesel::joinable!(clinic_services -> clinics (clinic_id));
diesel::joinable!(clinic_services -> health_services (service_id));
diesel::joinable!(health_service_variants -> health_services (service_id));
diesel::joinable!(health_services -> product_types (product_type_id));
diesel::joinable!(service_appointments -> clinic_service_time_slots (service_time_slot_id));
diesel::joinable!(service_appointments -> clinics (clinic_id));
diesel::joinable!(service_appointments -> health_service_variants (service_variant_id));

diesel::allow_tables_to_appear_in_same_query!(
    clinic_service_time_slots,
    clinic_services,
    clinics,
    health_service_variants,
    health_services,
    product_types,
    service_appointments,
);
