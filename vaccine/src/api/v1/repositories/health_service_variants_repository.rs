use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::{health_service_variants_model::*, response_model::Meta};

pub async fn get_health_service_variants(
    pool: &Pool<MySql>, 
    page: usize, 
    limit: usize, 
    service_id: usize
) -> Result<PaginatedHealthServiceVariants, Error> {
    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM health_service_variants
        WHERE service_id = ?
        "#,
        service_id as i32
    )
    .fetch_one(pool)
    .await?;

    let offset = page * limit;
    
    let health_service_variants = sqlx::query_as!(
        HealthServiceVariant,
        r#"
        SELECT 
            id,
            name,
            service_id,
            number_of_dose,
            shopify_id,
            shopify_sku,
            shopify_variant_id,
            price
        FROM health_service_variants
        WHERE service_id = ?
        ORDER BY id LIMIT ? OFFSET ? 
        "#,
        service_id as i32,
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await?;

    Ok(PaginatedHealthServiceVariants {
        data: health_service_variants,
        meta: Meta { total, page, limit, total_pages: (total as f64 / limit as f64).ceil() as usize }
    })}

pub async fn get_health_service_variant_by_id(
    pool: &Pool<MySql>, 
    id: i32
) -> Result<HealthServiceVariant, Error> {
    sqlx::query_as!(
        HealthServiceVariant,
        r#"
        SELECT 
            id,
            name,
            service_id,
            number_of_dose,
            shopify_id,
            shopify_sku,
            shopify_variant_id,
            price
        FROM health_service_variants
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn add_health_service_variant(
    pool: &Pool<MySql>, 
    health_service_variant: &CreateHealthServiceVariant, 
    user: i32
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO health_service_variants (
            service_id, name, number_of_dose, shopify_id, shopify_sku, shopify_variant_id, price, created_by, updated_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#,
        health_service_variant.service_id,
        health_service_variant.name,
        health_service_variant.number_of_dose,
        health_service_variant.shopify_id,
        health_service_variant.shopify_sku,
        health_service_variant.shopify_variant_id,
        health_service_variant.price,
        user,
        user
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn edit_health_service_variant(
    pool: &Pool<MySql>,
    id: i32,
    health_service_variant: &EditHealthServiceVariant,
    user_id: i32
) -> Result<Option<i32>, Error> {

    let data = sqlx::query!{
        r#"
        SELECT id
        FROM health_service_variants
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

    let mut query = String::from("UPDATE health_service_variants SET ");
    let mut sets = Vec::new();
    
    if let Some(ref name) = health_service_variant.name {
        sets.push(format!("name = '{}'", name));
    }
    if let Some(service_id) = health_service_variant.service_id {
        sets.push(format!("service_id = {}", service_id));
    }
    if let Some(ref number_of_dose) = health_service_variant.number_of_dose {
        sets.push(format!("number_of_dose = {}", number_of_dose));
    }
    if let Some(ref shopify_id) = health_service_variant.shopify_id {
        sets.push(format!("shopify_id = '{}'", shopify_id));
    }
    if let Some(ref shopify_sku) = health_service_variant.shopify_sku {
        sets.push(format!("shopify_sku = '{}'", shopify_sku));
    }
    if let Some(ref shopify_variant_id) = health_service_variant.shopify_variant_id {
        sets.push(format!("shopify_variant_id = '{}'", shopify_variant_id));
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

pub async fn delete_health_service_variant(
    pool: &Pool<MySql>, 
    id: i32, 
    user_id: i32
) -> Result<Option<i32>, Error> {
    let data = sqlx::query!{
        r#"
        SELECT id
        FROM health_service_variants
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
        DELETE FROM health_service_variants
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(Some(id))
}