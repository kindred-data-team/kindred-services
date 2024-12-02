use crate::db::create_db_connection;
use tokio_postgres::Error;
use crate::models::vaccine::Vaccine;

pub async fn insert_vaccine(vaccine: &Vaccine) -> Result<(), Error> {
    let client = create_db_connection().await?; // Async DB connection
    client.execute(
        "INSERT INTO vaccines (name, brand, details, for_whom, price, number_of_dose, code, shopify_id, shopify_sku, shopify_variant_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        &[&vaccine.name, 
          &vaccine.brand, 
          &vaccine.details, 
          &vaccine.for_whom, 
          &vaccine.price, 
          &vaccine.number_of_dose, 
          &vaccine.code, 
          &vaccine.shopify_id, 
          &vaccine.shopify_sku, 
          &vaccine.shopify_variant_id]
    ).await?; // Async query execution
    Ok(())
}

pub async fn get_vaccine_by_id(id: i32) -> Result<Vaccine, Error> {
    let  client = create_db_connection().await?; // Async DB connection
    let row = client.query_one(
        "SELECT * FROM vaccines WHERE id = $1", &[&id]
    ).await?; // Async query execution
    Ok(Vaccine {
        id: row.get(0),
        name: row.get(1),
        brand: row.get(2),
        details: row.get(3),
        for_whom: row.get(4),
        price: row.get(5),
        number_of_dose: row.get(6),
        code: row.get(7),
        shopify_id: row.get(8),
        shopify_sku: row.get(9),
        shopify_variant_id: row.get(10),
    })
}

pub async fn get_all_vaccines() -> Result<Vec<Vaccine>, Error> {
    let  client = create_db_connection().await?; // Async DB connection
    let rows = client.query("SELECT * FROM vaccines", &[]).await?; // Async query execution

    let vaccines: Vec<Vaccine> = rows.iter().map(|row| Vaccine {
        id: row.get(0),
        name: row.get(1),
        brand: row.get(2),
        details: row.get(3),
        for_whom: row.get(4),
        price: row.get(5),
        number_of_dose: row.get(6),
        code: row.get(7),
        shopify_id: row.get(8),
        shopify_sku: row.get(9),
        shopify_variant_id: row.get(10),
    }).collect();

    Ok(vaccines)
}

pub async fn update_vaccine(id: i32, vaccine: &Vaccine) -> Result<(), Error> {
    let  client = create_db_connection().await?; // Async DB connection
    client.execute(
        "UPDATE vaccines SET name = $2, brand = $3, details = $4, for_whom = $5, price = $6, number_of_dose = $7, code = $8, shopify_id = $9, shopify_sku = $10, shopify_variant_id = $11 WHERE id = $1",
        &[&id, 
          &vaccine.name, 
          &vaccine.brand, 
          &vaccine.details, 
          &vaccine.for_whom,
          &vaccine.price, 
          &vaccine.number_of_dose, 
          &vaccine.code, 
          &vaccine.shopify_id, 
          &vaccine.shopify_sku, 
          &vaccine.shopify_variant_id]
    ).await?; // Async query execution
    Ok(())
}

pub async fn delete_vaccine(id: i32) -> Result<(), Error> {
    let  client = create_db_connection().await?; // Async DB connection
    client.execute("DELETE FROM vaccines WHERE id = $1", &[&id]).await?; // Async query execution
    Ok(()) 
}