use sqlx::{MySql, Pool, Error};
use chrono::NaiveDate;
use crate::api::v1::models::{response_model::Meta, time_slots_model::*};

pub async fn get_all_time_slots(
    pool: &Pool<MySql>, 
    clinic_id: usize,
    date: NaiveDate
) -> Result<PaginatedTimeSlots, Error> {

    let time_slots = sqlx::query_as!(
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
        WHERE clinic_id = ? AND is_available = 1 AND date >= ?
        ORDER BY date, start_time
        "#,
        clinic_id as i64,
        date
    )
    .fetch_all(pool)
    .await?;

    // Group the time slots by date
    let mut grouped_slots: std::collections::HashMap<chrono::NaiveDate, GroupedTimeSlots> = std::collections::HashMap::new();
    
    for slot in time_slots {
        if let Some(date) = slot.date {
            let simple_slot = TimeSlotData {
                id: slot.id,
                start_time: slot.start_time,
                end_time: slot.end_time,
                is_available: slot.is_available,
                is_virtual: slot.is_virtual,
                number_of_slots: slot.number_of_slots
            };
            
            grouped_slots.entry(date)
                .or_insert_with(|| GroupedTimeSlots {
                    date,
                    clinic_id: slot.clinic_id,
                    slots: Vec::new(),
                })
                .slots.push(simple_slot);
        }
    }

    Ok(PaginatedTimeSlots {
        data: grouped_slots.into_values().collect(),
        meta: Meta {
            total: 0,
            page: 0,
            limit: 0,
            total_pages: 0
        }
    })
}