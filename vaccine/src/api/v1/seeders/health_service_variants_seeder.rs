use actix_web::web;
use sqlx::{MySqlPool, Error};

pub async fn seed_health_service_variants(pool: web::Data<MySqlPool>) -> Result<(), Error> {
    let health_service_variants = vec![
        (1, 1, "Gardasil 4 Single", 1, 1000, "Gardasil 4 Single Shopify ID", "Gardasil 4 Single Shopify SKU", "Gardasil 4 Single Shopify Variant ID"),
        (2, 1, "Gardasil 4 Triple", 3, 3000, "Gardasil 4 Triple Shopify ID", "Gardasil 4 Triple Shopify SKU", "Gardasil 4 Triple Shopify Variant ID"),
        (3, 1, "Gardasil 9 Single", 1, 1000, "Gardasil 9 Single Shopify ID", "Gardasil 9 Single Shopify SKU", "Gardasil 9 Single Shopify Variant ID"),
        (4, 1, "Gardasil 9 Triple", 3, 3000, "Gardasil 9 Triple Shopify ID", "Gardasil 9 Triple Shopify SKU", "Gardasil 9 Triple Shopify Variant ID"),
        (5, 2, "Fluarix", 1, 1000, "Fluarix Shopify ID", "Fluarix Shopify SKU", "Fluarix Shopify Variant ID"),
        (6, 2, "Vaxigrip", 1, 1000, "Vaxigrip Shopify ID", "Vaxigrip Shopify SKU", "Vaxigrip Shopify Variant ID"),
        (7, 2, "FluQuadri", 1, 1000, "FluQuadri Shopify ID", "FluQuadri Shopify SKU", "FluQuadri Shopify Variant ID"),
        (8, 3, "Genevac B Single", 1, 1000, "Genevac B Single Shopify ID", "Genevac B Single Shopify SKU", "Genevac B Single Shopify Variant ID"),
        (9, 3, "Genevac B Triple", 3, 3000, "Genevac B Triple Shopify ID", "Genevac B Triple Shopify SKU", "Genevac B Triple Shopify Variant ID"),
        (10, 3, "Engerix Single", 1, 1000, "Engerix Single Shopify ID", "Engerix Single Shopify SKU", "Engerix Single Shopify Variant ID"),
        (11, 3, "Engerix Triple", 1, 3000, "Engerix Triple Shopify ID", "Engerix Triple Shopify SKU", "Engerix Triple Shopify Variant ID"),
        (12, 4, "Shingrix", 1, 1000, "Shingrix Shopify ID", "Shingrix Shopify SKU", "Shingrix Shopify Variant ID"),
        (13, 5, "DTAP", 1, 1000, "DTAP Shopify ID", "DTAP Shopify SKU", "DTAP Shopify Variant ID"),
        (14, 6, "Prevnar 13", 1, 1000, "Prevnar 13 Shopify ID", "Prevnar 13 Shopify SKU", "Prevnar 13 Shopify Variant ID"),
        (15, 6, "Pneumovax 23", 1, 1000, "Pneumovax 23 Shopify ID", "Pneumovax 23 Shopify SKU", "Pneumovax 23 Shopify Variant ID"),
        (16, 7, "DMPA", 1, 1000, "DMPA Shopify ID", "DMPA Shopify SKU", "DMPA Shopify Variant ID")
    ];

    for (id, health_service_id, name, number_of_dose, price, shopify_id, shopify_sku, shopify_variant_id) in health_service_variants {        
        sqlx::query!(
            r#"
            REPLACE INTO health_service_variants (id, service_id, name, number_of_dose, price, shopify_id, shopify_sku, shopify_variant_id, created_by, updated_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, 1, NOW(), NOW())
            "#,
            id,
            health_service_id,
            name,
            number_of_dose,
            price,
            shopify_id,
            shopify_sku,
            shopify_variant_id
        )
        .execute(pool.get_ref())
        .await?;
    }

    println!("Health services variants seeded successfully!");
    Ok(())
}
