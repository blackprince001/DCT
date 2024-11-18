mod network;
mod server;

use network::{
    db::SqliteStorage,
    types::{DataSize, NetworkAnalytics},
};
use server::http::create_router;
use std::{error::Error, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use sysinfo::Networks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let networks = Networks::new_with_refreshed_list();
    let interface = String::from("en0");
    let scrape_time = 1000; // 1 second
    let packet_size = DataSize::Kilobyte;

    // Create shared state
    let analytics = NetworkAnalytics::new(interface.clone(), packet_size);
    let shared_state = Arc::new(RwLock::new(analytics));

    // Create an Sqlite Data Storage db
    let db = SqliteStorage::new("metrics.db").await?;

    // Create router
    let app = create_router(shared_state.clone(), db);

    // Start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on http://{}", addr);

    let server_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server listening on {}", addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    let network_handle = tokio::spawn({
        let shared_state = shared_state.clone();
        async move {
            if let Err(e) = network::net::run(networks, &interface, scrape_time, shared_state).await
            {
                eprintln!("Network monitoring error: {}", e);
            }
        }
    });

    tokio::select! {
        _ = server_handle => println!("Server task stopped"),
        _ = network_handle => println!("Network task stopped"),
    }

    Ok(())
}
