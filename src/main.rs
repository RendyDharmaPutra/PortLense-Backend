mod api;
mod services;
mod core;

use std::env;

use axum::{routing::get, Router};
use api::handlers::port_handlers::ports_handler;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").expect("PORT not found in .env");

    let app = Router::new().route("/ports", get(ports_handler));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}