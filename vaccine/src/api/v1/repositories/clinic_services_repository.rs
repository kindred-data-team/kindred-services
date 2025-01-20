use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::{clinic_services_model::*, response_model::Meta};

pub async fn get_clinic_services(
    pool: &Pool<MySql>, 
    page: usize, 
    limit: usize, 
    service_id: usize
) -> Result<PaginatedClinicServices, Error> {
    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM clinic_services
        WHERE service_id = ?
        "#,
        service_id as i32
    )
    .fetch_one(pool)
    .await?;

    let offset = page * limit;
    
    let clinic_services = sqlx::query_as!(
        ClinicService,
        r#"
        SELECT 
            cs.id AS 'id',
            cs.clinic_id AS 'clinic_id',
            cs.service_id AS 'service_id',
            c.name AS 'clinic_name',
            c.availability_time AS 'clinic_availability_time'
        FROM clinic_services cs
        LEFT JOIN clinics c ON cs.clinic_id = c.id
        LEFT JOIN health_services s ON cs.service_id = s.id
        WHERE cs.service_id = ?
        ORDER BY c.name LIMIT ? OFFSET ? 
        "#,
        service_id as i32,
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await?;

    Ok(PaginatedClinicServices {
        data: clinic_services,
        meta: Meta { total, page, limit, total_pages: (total as f64 / limit as f64).ceil() as usize }
    })}

pub async fn add_clinic_service(
    pool: &Pool<MySql>, 
    clinic_service: &CreateClinicService, 
    user: i32
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO clinic_services (
            clinic_id, service_id, created_by, updated_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, NOW(), NOW())
        "#,
        clinic_service.clinic_id,
        clinic_service.service_id,
        user,
        user
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn edit_clinic_service(
    pool: &Pool<MySql>,
    id: i32,
    clinic_service: &EditClinicService,
    user_id: i32
) -> Result<Option<i32>, Error> {

    let data = sqlx::query!{
        r#"
        SELECT id
        FROM clinic_services
        WHERE (id = ? AND created_by = ?)
        "#,
        id,
        user_id
    }
    .fetch_optional(pool)
    .await?;

    if data.is_none() {
        return Ok(None);
    }

    let mut query = String::from("UPDATE clinic_services SET ");
    let mut sets = Vec::new();
    
    if let Some(ref clinic_id) = clinic_service.clinic_id {
        sets.push(format!("clinic_id = {}", clinic_id));
    }
    if let Some(service_id) = clinic_service.service_id {
        sets.push(format!("service_id = {}", service_id));
    }

    sets.push(format!("updated_by = {}", user_id));
    sets.push("updated_at = NOW()".to_string());
    
    if sets.is_empty() {
        return Ok(Some(id));
    }
    
    query.push_str(&sets.join(", "));
    query.push_str(&format!(" WHERE id = {}", id));

    sqlx::query(&query)
        .execute(pool)
        .await?;
        
    Ok(Some(id))
}

pub async fn delete_clinic_service(
    pool: &Pool<MySql>, 
    id: i32, 
    user_id: i32
) -> Result<Option<i32>, Error> {
    let data = sqlx::query!{
        r#"
        SELECT id
        FROM clinic_services
        WHERE (id = ? AND created_by = ?)
        "#,
        id,
        user_id
    }
    .fetch_optional(pool)
    .await?;

    if data.is_none() {
        return Ok(None);
    }

    sqlx::query!(
        r#"
        DELETE FROM clinic_services
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(Some(id))
}