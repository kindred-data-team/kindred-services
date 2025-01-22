use actix_web::web;
use sqlx::{MySqlPool, Error};
use crate::api::v1::seeders::{*};

pub async fn run_seeder(pool: web::Data<MySqlPool>) -> Result<(), Error> {
    product_types_seeder::seed_product_types(pool.clone()).await?;
    health_services_seeder::seed_health_services(pool.clone()).await?;
    health_service_variants_seeder::seed_health_service_variants(pool.clone()).await?;
    println!("Database seeded successfully!");
    Ok(())
}