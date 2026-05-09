mod api;
mod services;
mod core;

use axum::{routing::get, Router};
use api::handlers::ports_handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ports", get(ports_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}