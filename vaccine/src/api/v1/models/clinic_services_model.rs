use serde::{Deserialize, Serialize};
use validator::Validate;

use super::response_model::Meta;

#[derive(Serialize, Deserialize, Validate)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub service_id: usize
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ClinicService {
    pub id: Option<i32>,
    pub clinic_id: Option<i32>,
    pub service_id: Option<i32>,
    pub clinic_name: Option<String>,
    pub clinic_availability_time: Option<String>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditClinicService {
    pub clinic_id: Option<i32>,
    pub service_id: Option<i32>,
    pub plato_code: Option<String>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateClinicService {
    pub clinic_id: i32,
    pub service_id: i32,
    pub plato_code: String
}

#[derive(Debug, Serialize)]
pub struct PaginatedClinicServices {
    pub data: Vec<ClinicService>,
    pub meta: Meta
}