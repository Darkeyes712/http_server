mod http_server;
use tokio::signal::ctrl_c;
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawning the HTTP server using tokio::spawn()
    let task = task::spawn(http_server::run_http_server(3000));
    let task1 = task::spawn(http_server::run_http_server(3001));

    // Wait for the Ctrl+C signal to terminate the server gracefully
    ctrl_c().await.expect("Failed to listen for Ctrl+C signal");

    // Print a message indicating that the server is shutting down
    println!("Shutting down the servers...");

    // Wait for the server task to complete gracefully
    task.await.unwrap();

    // Once the server task completes, the process will terminate gracefully
    println!("Servers have been shut down.");
}