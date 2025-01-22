use actix_web::web;
use sqlx::{MySqlPool, Error};

pub async fn seed_health_services(pool: web::Data<MySqlPool>) -> Result<(), Error> {
    let health_services = vec![
        (1, 2, "HPV Vaccine", "HPV Vaccine Description", "HPV Vaccine Details", "HPV Vaccine For Whom", "HPV Vaccine Shopify ID", "HPV Vaccine Shopify SKU", "HPV Vaccine Shopify Variant ID", "HPV Vaccine Image URL"),
        (2, 2, "Flu Vaccine", "Flu Vaccine Description", "Flu Vaccine Details", "Flu Vaccine For Whom", "Flu Vaccine Shopify ID", "Flu Vaccine Shopify SKU", "Flu Vaccine Shopify Variant ID", "Flu Vaccine Image URL"),
        (3, 2, "Hepatits B Vaccine", "Hepatits B Vaccine Description", "Hepatits B Vaccine Details", "Hepatits B Vaccine For Whom", "Hepatits B Vaccine Shopify ID", "Hepatits B Vaccine Shopify SKU", "Hepatits B Vaccine Shopify Variant ID", "Hepatits B Vaccine Image URL"),
        (4, 2, "Shingles Vaccine", "Shingles Vaccine Description", "Shingles Vaccine Details", "Shingles Vaccine For Whom", "Shingles Vaccine Shopify ID", "Shingles Vaccine Shopify SKU", "Shingles Vaccine Shopify Variant ID", "Shingles Vaccine Image URL"),
        (5, 2, "DTAP Vaccine", "DTAP Vaccine Description", "DTAP Vaccine Details", "DTAP Vaccine For Whom", "DTAP Vaccine Shopify ID", "DTAP Vaccine Shopify SKU", "DTAP Vaccine Shopify Variant ID", "DTAP Vaccine Image URL"),
        (6, 2, "Pneumonia Vaccine", "Pneumonia Vaccine Description", "Pneumonia Vaccine Details", "Pneumonia Vaccine For Whom", "Pneumonia Vaccine Shopify ID", "Pneumonia Vaccine Shopify SKU", "Pneumonia Vaccine Shopify Variant ID", "Pneumonia Vaccine Image URL"),
        (7, 2, "DMPA Vaccine", "DMPA Vaccine Description", "DMPA Vaccine Details", "DMPA Vaccine For Whom", "DMPA Vaccine Shopify ID", "DMPA Vaccine Shopify SKU", "DMPA Vaccine Shopify Variant ID", "DMPA Vaccine Image URL"),
    ];

    for (id, product_type_id, name, description, details, for_whom, shopify_id, shopify_sku, shopify_variant_id, image_url) in health_services {        
        sqlx::query!(
            r#"
            REPLACE INTO health_services (id, product_type_id, name, description, details, for_whom, shopify_id, shopify_sku, shopify_variant_id, image_url, created_by, updated_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 1, NOW(), NOW())
            "#,
            id,
            product_type_id,
            name,
            description,
            details,
            for_whom,
            shopify_id,
            shopify_sku,
            shopify_variant_id,
            image_url
        )
        .execute(pool.get_ref())
        .await?;
    }

    println!("Health services seeded successfully!");
    Ok(())
}
