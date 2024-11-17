mod network;
mod server;

use std::error::Error;
use tokio::net::TcpListener;

use sysinfo::Networks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let networks = Networks::new_with_refreshed_list();
    let interface = String::from("en0");
    let scrape_time = 1000; // 1 second
    let packetsize = network::types::DataSize::Kilobyte;

    let app = server::http::create_router();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));

    let server_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server listening on {}", addr);
        axum::serve(listener, app).await.unwrap();
    });

    let network_handle = tokio::spawn(async move {
        if let Err(e) = network::net::run(networks, &interface, scrape_time, packetsize) {
            eprintln!("Network monitoring error: {}", e);
        }
    });

    tokio::select! {
        _ = server_handle => println!("Server task completed"),
        _ = network_handle => println!("Network task completed"),
    }

    Ok(())
}
