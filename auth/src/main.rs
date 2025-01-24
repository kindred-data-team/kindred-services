#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use clap::Parser;
use seeders::seed_db::seeder;

use crate::routes::auth::auth_routes;
use crate::middleware::logger::Logger;

mod models;
mod api;
mod routes;
mod repository;
mod db;
mod config;
mod middleware;
mod schema;
mod utils;
mod seeders;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run the roles seeder
    #[arg(short, long)]
    seed: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let host = config::config::get_host();
    let port = config::config::get_port();

    let args = Args::parse();

    if args.seed {
        seeder();
        std::process::exit(0);
    }

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