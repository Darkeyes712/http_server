use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Maika ti she eva, backa si survura kak si trebe!"
}

pub async fn run_http_server(port: u16) {
    // Create a new Axum router
    let app = Router::new().route("/", get(hello_world));

    // Set the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Clone the address to print it in the spawned task
    let print_addr = addr.clone();

    // Spawn the server in a new task
    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        // This will be executed when the server is shut down
        println!("Server on port {} has been shut down.", print_addr.port());
    });

    // Print the server's address after starting it
    println!("Server running at http://{}", addr);
}