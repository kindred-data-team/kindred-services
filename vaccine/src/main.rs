use actix_web::{web, App, HttpServer};
use crate::api::v1::routes::routes::routes;
use crate::api::v1::middleware::logger::Logger;
use crate::api::v1::middleware::authentication::JwtAuth;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

mod api;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Seed,
    Serve,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    let pool = api::v1::db::database::create_db_pool().await;
    let pool_data = web::Data::new(pool);

    match &cli.command {
        Some(Commands::Seed) => {
            println!("Running database seeder...");

            
            if let Err(e) = api::v1::seeders::seeders::run_seeder(pool_data).await {
                eprintln!("Error seeding database: {}", e);
                std::process::exit(1);
            }
            
            println!("Seeding completed successfully!");
            std::process::exit(0);
        }
        Some(Commands::Serve) | None => {
            let host = api::v1::config::config::get_host();
            let port = api::v1::config::config::get_port();

            println!("Server running at http://{}:{}", host, port);
            HttpServer::new(move || {
                App::new()
                    .wrap(Logger)
                    .wrap(JwtAuth)
                    .service(
                        routes(pool_data.clone())
                    )
            })
            .bind((host, port.parse::<u16>().unwrap()))?
            .run()
            .await
        }
    }
}
