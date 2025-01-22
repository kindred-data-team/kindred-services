use actix_web::web;
use sqlx::{MySqlPool, Error};

pub async fn seed_product_types(pool: web::Data<MySqlPool>) -> Result<(), Error> {
    let product_types = vec![
        (1, "Consultation"),
        (2, "Vaccine")
    ];

    for (id, product_type) in product_types {        
        sqlx::query!(
            r#"
            REPLACE INTO product_types (id, name, created_by, updated_by, created_at, updated_at)
            VALUES (?, ?, 1, 1, NOW(), NOW())
            "#,
            id,
            product_type,
        )
        .execute(pool.get_ref())
        .await?;
    }

    println!("Product types seeded successfully!");
    Ok(())
}
