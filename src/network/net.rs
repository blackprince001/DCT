use crate::network::types::NetworkAnalytics;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::Networks;
use tokio::sync::RwLock;
use tokio::time;

pub async fn run(
    mut networks: Networks,
    interface: &str,
    scrape_interval: u64,
    analytics: Arc<RwLock<NetworkAnalytics>>,
) -> Result<(), Box<dyn Error>> {
    let mut interval = time::interval(Duration::from_millis(scrape_interval));

    loop {
        networks.refresh();

        if let Some(network) = networks.iter().find(|(name, _)| *name == interface) {
            let (_, network_data) = network;

            let mut analytics = analytics.write().await;
            analytics.update_from_sysinfo(
                network_data.total_received(),
                network_data.total_transmitted(),
            )
        }

        interval.tick().await;
    }
}
