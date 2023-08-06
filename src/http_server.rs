use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, this is a simple HTTP server with Axum!"
}

pub async fn run_server() {
    // Create a new Axum router
    let app = Router::new().route("/", get(hello_world));

    // Set the server address and run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}