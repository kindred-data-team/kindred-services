#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use crate::routes::auth::auth_routes;
// use dotenv::dotenv;
use crate::middleware::logger::Logger;

mod models;
mod api;
mod routes;
mod repository;
mod db;
mod config;
mod middleware;
mod schema;
mod helper;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
 
    let host = config::config::get_host();
    let port = config::config::get_port();

    println!("Server running at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
        .wrap(Logger)
            .service(auth_routes()) 
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}