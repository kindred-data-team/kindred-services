use actix_web::{web, App, HttpServer};
use crate::routes::vaccine::vaccine_routes;
use dotenv::dotenv;
use crate::middleware::logger::Logger;

mod models;
mod api;
mod routes;
mod repository;
mod db;
mod config;
mod middleware;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
 
    let host = config::config::get_host();
    let port = config::config::get_port();

    let db_pool = db::create_db_pool().await;

    let pool_data = web::Data::new(db_pool);

    println!("Server running at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
        .wrap(Logger)
            .service(vaccine_routes(pool_data.clone())) // Pass the pool to vaccine_routes
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}