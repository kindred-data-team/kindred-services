use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use crate::api::v1::config::config;

pub async fn create_db_pool() -> MySqlPool {
    let database_url = config::get_database_url();
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}