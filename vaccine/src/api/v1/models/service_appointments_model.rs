use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{NaiveDate, NaiveTime};

use super::response_model::Meta;

#[derive(Serialize, Deserialize, Validate)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub page: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ServiceAppointment {
    pub id: i64,
    pub patient_id: Option<i64>,
    pub doctor_id: Option<i64>,
    pub dosage_number: Option<i64>,
    pub status: Option<String>,
    pub clinic_name: Option<String>,
    pub service_variant_name: Option<String>,
    pub service_name: Option<String>,
    pub product_type_name: Option<String>,
    pub appointment_date: Option<NaiveDate>,
    pub appointment_start_time: Option<NaiveTime>,
    pub appointment_end_time: Option<NaiveTime>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateServiceAppointment {
    pub service_variant_id: i64,
    pub service_time_slot_id: i64,
    pub doctor_id: Option<i64>,
    pub dosage_number: i64,
    pub status: String
}

#[derive(Debug, serde::Serialize)]
pub struct PaginatedServiceAppointments {
    pub data: Vec<ServiceAppointment>,
    pub meta: Meta
}