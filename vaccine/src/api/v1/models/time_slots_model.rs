use serde::{Deserialize, Serialize};
use validator::Validate;

use super::response_model::Meta;

#[derive(Serialize, Deserialize, Validate)]
pub struct FilterOptions {
    pub clinic_id: usize,
    pub date: String
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct TimeSlot {
    pub id: i64,
    pub clinic_id: Option<i64>,
    pub date: Option<chrono::NaiveDate>,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub is_available: Option<i8>,
    pub is_virtual: Option<i8>,
    pub number_of_slots: Option<i64>
}

#[derive(Debug, serde::Serialize)]
pub struct TimeSlotData {
    pub id: i64,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub is_available: Option<i8>,
    pub is_virtual: Option<i8>,
    pub number_of_slots: Option<i64>
}

#[derive(Debug, Serialize)]
pub struct GroupedTimeSlots {
    pub date: chrono::NaiveDate,
    pub clinic_id: Option<i64>,
    pub slots: Vec<TimeSlotData>,
}

#[derive(Debug, serde::Serialize)]
pub struct PaginatedTimeSlots {
    pub data: Vec<GroupedTimeSlots>,
    pub meta: Meta
}