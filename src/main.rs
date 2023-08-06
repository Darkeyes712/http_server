mod http_server;


#[tokio::main]
async fn main() {
    println!("Server is running...");
    http_server::run_server().await;
    println!("Server stopped running.");
}
