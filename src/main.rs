use axum::Router;
use tokio;
use std::net::SocketAddr;
use crate::routes::vaccine::vaccine_routes;
mod models;
mod api;
mod routes;
mod repository;
mod db;
mod constants;



#[tokio::main]
async fn main() {
    // Initialize environment and routes
    let app = Router::new().nest("/api", vaccine_routes());

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

