use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::{health_services_model::*, response_model::Meta};

pub async fn get_health_services(pool: &Pool<MySql>, page: usize, limit: usize, product_type_id: usize) -> Result<PaginatedHealthServices, Error> {
    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM health_services
        WHERE product_type_id = ?
        "#,
        product_type_id as i32
    )
    .fetch_one(pool)
    .await?;

    let offset = (page - 1) * limit;
    let health_services = sqlx::query_as!(
        HealthService,
        r#"
        SELECT
            hs.id,
            hs.product_type_id,
            hs.name,
            hs.description,
            hs.details,
            hs.for_whom,
            hs.shopify_id,
            hs.shopify_sku,
            hs.shopify_variant_id,
            hs.image_url,
            MIN(hsv.price) AS "min_price",
            MAX(hsv.price) AS "max_price"
        FROM health_services as hs
        LEFT JOIN health_service_variants as hsv ON hs.id = hsv.service_id
        WHERE hs.product_type_id = ?
        GROUP BY hs.id 
        ORDER BY hs.id LIMIT ? OFFSET ?
        "#,
        product_type_id as i32,
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await?;

    Ok(PaginatedHealthServices {
        data: health_services,
        meta: Meta { total, page, limit, total_pages: (total as f64 / limit as f64).ceil() as usize }
    })
}

pub async fn get_health_service_by_id(pool: &Pool<MySql>, id: i32) -> Result<HealthService, Error> {
    sqlx::query_as!(
        HealthService,
        r#"
        SELECT 
            hs.id,
            hs.product_type_id,
            hs.name,
            hs.description,
            hs.details,
            hs.for_whom,
            hs.shopify_id,
            hs.shopify_sku,
            hs.shopify_variant_id,
            hs.image_url,
            MIN(hsv.price) AS "min_price",
            MAX(hsv.price) AS "max_price"  
        FROM health_services as hs
        JOIN health_service_variants as hsv ON hs.id = hsv.service_id
        WHERE hs.id = ?
        GROUP BY hs.id 
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn add_health_service(
    pool: &Pool<MySql>, 
    service: &CreateHealthService, 
    user: i32
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO health_services (
            product_type_id, name, description, details, for_whom, shopify_id, shopify_sku, shopify_variant_id, image_url, created_by, updated_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#,
        service.product_type_id,
        service.name,
        service.description,
        service.details,
        service.for_whom,
        service.shopify_id,
        service.shopify_sku,
        service.shopify_variant_id,
        service.image_url,
        user,
        user
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn edit_health_service(
    pool: &Pool<MySql>,
    id: i32,
    health_service: &EditHealthService,
    user_id: i32
) -> Result<(), Error> {
    let mut query = String::from("UPDATE health_services SET ");
    let mut sets = Vec::new();
    
    if let Some(ref name) = health_service.name {
        sets.push(format!("name = '{}'", name));
    }
    if let Some(product_type_id) = health_service.product_type_id {
        sets.push(format!("product_type_id = {}", product_type_id));
    }
    if let Some(ref details) = health_service.details {
        sets.push(format!("details = '{}'", details));
    }
    if let Some(ref shopify_id) = health_service.shopify_id {
        sets.push(format!("shopify_id = {}", shopify_id));
    }
    if let Some(ref shopify_sku) = health_service.shopify_sku {
        sets.push(format!("shopify_sku = '{}'", shopify_sku));
    }
    if let Some(ref shopify_variant_id) = health_service.shopify_variant_id {
        sets.push(format!("shopify_variant_id = {}", shopify_variant_id));
    }
    if let Some(ref for_whom) = health_service.for_whom {
        sets.push(format!("for_whom = '{}'", for_whom));
    }
    if let Some(ref description) = health_service.description {
        sets.push(format!("description = '{}'", description));
    }
    
    sets.push(format!("updated_by = {}", user_id));
    sets.push("updated_at = NOW()".to_string());
    
    if sets.is_empty() {
        return Ok(());
    }
    
    query.push_str(&sets.join(", "));
    query.push_str(&format!(" WHERE id = {}", id));

    sqlx::query(&query)
        .execute(pool)
        .await?;
        
    Ok(())
}

pub async fn delete_health_service(pool: &Pool<MySql>, id: i32, user_id: i32) -> Result<Option<i32>, Error> {
    let data = sqlx::query!{
        r#"
        SELECT id
        FROM health_services
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
        DELETE FROM health_services
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(Some(id))
}