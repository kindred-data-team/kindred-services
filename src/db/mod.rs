use dotenv::dotenv;
use std::env;

use tokio_postgres::{Client, NoTls, Error};

pub async fn create_db_connection() -> Result<Client, Error> {
    dotenv().ok();
    let host = env::var("DB_HOST");
    let user = env::var("DB_USER");
    let password = env::var("DB_PASSWORD");
    let dbname = env::var("DB_NAME");

    // /"host=localhost user=postgres password=1234 dbname=kindred-rust"
    let db_config = format!( "host={} user={} password={} dbname={}", host.unwrap(), user.unwrap(), password.unwrap(), dbname.unwrap());
    
    let (client, connection) = tokio_postgres::connect(&db_config, NoTls).await?;

    // Spawn the connection task to run in the background
    tokio::spawn(connection);
    
    Ok(client)
}