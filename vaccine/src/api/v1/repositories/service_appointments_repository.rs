use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::service_appointments_model::*;
use crate::api::v1::models::response_model::*;
use crate::api::v1::models::time_slots_model::*;

pub async fn get_user_service_appointments(
    pool: &Pool<MySql>, 
    page: usize, 
    limit: usize,
    user_id: i32
) -> Result<PaginatedServiceAppointments, Error> {
    // Get total count
    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM service_appointments WHERE (patient_id = ? or doctor_id = ?)",
        user_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    let offset = (page - 1) * limit;
    let service_appointments = sqlx::query_as!(
        ServiceAppointment,
        r#"
        SELECT 
            sa.id AS 'id: i64',
            sa.patient_id AS 'patient_id: i64', 
            sa.dosage_number AS 'dosage_number: i64', 
            sa.doctor_id AS 'doctor_id: i64', 
            sa.status AS 'status',
            c.name AS 'clinic_name', 
            sv.name AS 'service_variant_name',
            s.name AS 'service_name',
            p.name AS 'product_type_name',
            ts.date AS 'appointment_date',
            ts.start_time AS 'appointment_start_time',
            ts.end_time AS 'appointment_end_time'
        FROM service_appointments sa
        LEFT JOIN clinics c ON sa.clinic_id = c.id
        LEFT JOIN health_service_variants sv ON sa.service_variant_id = sv.id
        LEFT JOIN health_services s ON sv.service_id = s.id
        LEFT JOIN product_types p ON s.product_type_id = p.id
        LEFT JOIN clinic_service_time_slots ts ON sa.service_time_slot_id = ts.id
        WHERE (sa.patient_id = ? OR sa.doctor_id = ?)
        ORDER BY ts.date DESC, ts.start_time DESC 
        LIMIT ? 
        OFFSET ? 
        "#,
        user_id,
        user_id,
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await?;
    
    Ok(PaginatedServiceAppointments {
        data: service_appointments,
        meta: Meta {
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as usize
        }
    })
}

pub async fn get_service_appointment_by_id(
    pool: &Pool<MySql>,
    id: i32,
    user_id: i32
) -> Result<ServiceAppointment, Error> {
    sqlx::query_as!(
        ServiceAppointment,
        r#"
        SELECT 
          sa.id AS 'id: i64',
            sa.patient_id AS 'patient_id: i64', 
            sa.dosage_number AS 'dosage_number: i64', 
            sa.doctor_id AS 'doctor_id: i64', 
            sa.status AS 'status',
            c.name AS 'clinic_name', 
            sv.name AS 'service_variant_name',
            s.name AS 'service_name',
            p.name AS 'product_type_name',
            ts.date AS 'appointment_date',
            ts.start_time AS 'appointment_start_time',
            ts.end_time AS 'appointment_end_time'
        FROM service_appointments sa
        LEFT JOIN clinics c ON sa.clinic_id = c.id
        LEFT JOIN health_service_variants sv ON sa.service_variant_id = sv.id
        LEFT JOIN health_services s ON sv.service_id = s.id
        LEFT JOIN product_types p ON s.product_type_id = p.id
        LEFT JOIN clinic_service_time_slots ts ON sa.service_time_slot_id = ts.id
        WHERE (sa.patient_id = ? OR sa.doctor_id = ?) AND sa.id = ?
        "#,
        user_id,
        user_id,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn add_service_appointment(
    pool: &Pool<MySql>, 
    service_appointment: &CreateServiceAppointment, 
    user_id: i32
) -> Result<(), Box<dyn std::error::Error>> {
    // (PATCH) Add database transaction 

    // Check time slot data, verify if there are still remaining slots
    let mut time_slot_data = sqlx::query_as!(
        TimeSlot,
        r#"
        SELECT 
            id AS `id: i64`,
            clinic_id AS `clinic_id: i64`,
            date,
            start_time,
            end_time,
            is_available,
            is_virtual,
            number_of_slots AS `number_of_slots: i64`
        FROM clinic_service_time_slots
        WHERE id = ?
        "#,
        service_appointment.service_time_slot_id
    )
    .fetch_one(pool)
    .await?;

    if time_slot_data.number_of_slots <= Some(0) {
        return Err("No available slot".into());
    }

    // Add appointment
    sqlx::query!(
        r#"
        INSERT INTO service_appointments (
            patient_id, clinic_id, service_variant_id, service_time_slot_id, doctor_id, dosage_number, status, created_by, updated_by
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        user_id,
        time_slot_data.clinic_id,
        service_appointment.service_variant_id,
        service_appointment.service_time_slot_id,
        service_appointment.doctor_id,
        service_appointment.dosage_number,
        service_appointment.status,
        user_id,
        user_id
    )
    .execute(pool)
    .await?;

    // Decrease number of slot
    time_slot_data.number_of_slots = Some(time_slot_data.number_of_slots.unwrap() - 1);

    sqlx::query!(
        r#"
        UPDATE clinic_service_time_slots
        SET number_of_slots = ?, updated_at = NOW()
        WHERE id = ?
        "#,
        time_slot_data.number_of_slots,
        time_slot_data.id
    )
    .execute(pool)
    .await?;
    Ok(())
}