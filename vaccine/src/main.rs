use actix_web::{web, App, HttpServer};
use crate::api::v1::routes::routes::routes;
use crate::api::v1::middleware::logger::Logger;
use crate::api::v1::middleware::authentication::JwtAuth;
use dotenv::dotenv;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = api::v1::config::config::get_host();
    let port = api::v1::config::config::get_port();
    let db_pool = api::v1::db::database::create_db_pool().await;

    let db = web::Data::new(db_pool);

    println!("Server running at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .wrap(JwtAuth)
            .service(
                routes(db.clone())
            )
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
