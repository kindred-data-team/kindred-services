use sqlx::{MySql, Pool, Error};
use crate::models::vaccine::Vaccine;


pub async fn insert_vaccine(pool: &Pool<MySql>, vaccine: &Vaccine) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO vaccine (
            name, brand, details, for_whom, price, number_of_dose, code, 
            shopify_id, shopify_sku, shopify_variant_id, variant_id, 
            created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#,
        vaccine.name,
        vaccine.brand,
        vaccine.details,
        vaccine.for_whom,
        vaccine.price,
        vaccine.number_of_dose,
        vaccine.code,
        vaccine.shopify_id,
        vaccine.shopify_sku,
        vaccine.shopify_variant_id,
        vaccine.variant_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}


pub async fn get_vaccine_by_id(pool: &Pool<MySql>, id: i32) -> Result<Vaccine, Error> {
    sqlx::query_as!(
        Vaccine,
        r#"
        SELECT id, name, brand, details, for_whom, price, number_of_dose, code, 
               shopify_id, shopify_sku, shopify_variant_id, variant_id, 
               created_at, updated_at
        FROM vaccine
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}


pub async fn get_all_vaccines(pool: &Pool<MySql>) -> Result<Vec<Vaccine>, Error> {
    sqlx::query_as!(
        Vaccine,
        r#"
        SELECT id, name, brand, details, for_whom, price, number_of_dose, code, 
               shopify_id, shopify_sku, shopify_variant_id, variant_id, 
               created_at, updated_at
        FROM vaccine
        "#
    )
    .fetch_all(pool)
    .await
}


pub async fn update_vaccine(pool: &Pool<MySql>, id: i32, vaccine: &Vaccine) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE vaccine
        SET name = ?, brand = ?, details = ?, for_whom = ?, price = ?, number_of_dose = ?, 
            code = ?, shopify_id = ?, shopify_sku = ?, shopify_variant_id = ?, 
            variant_id = ?, updated_at = NOW()
        WHERE id = ?
        "#,
        vaccine.name,
        vaccine.brand,
        vaccine.details,
        vaccine.for_whom,
        vaccine.price,
        vaccine.number_of_dose,
        vaccine.code,
        vaccine.shopify_id,
        vaccine.shopify_sku,
        vaccine.shopify_variant_id,
        vaccine.variant_id,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}


pub async fn delete_vaccine(pool: &Pool<MySql>, id: i32) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM vaccine
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}