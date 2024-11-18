use crate::network::types::NetworkAnalytics;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::Networks;
use tokio::sync::RwLock;
use tokio::time;

use super::db::{MetricsStorage, SqliteStorage};

pub async fn run(
    mut networks: Networks,
    interface: &str,
    scrape_interval: u64,
    analytics: Arc<RwLock<NetworkAnalytics>>,
    database: SqliteStorage,
) -> Result<(), Box<dyn Error>> {
    let mut interval = time::interval(Duration::from_millis(scrape_interval));

    loop {
        networks.refresh();

        if let Some(network) = networks.iter().find(|(name, _)| *name == interface) {
            let (_, network_data) = network;

            // Update analytics with new data
            let mut analytics = analytics.write().await;
            analytics.update_from_sysinfo(
                network_data.total_received(),
                network_data.total_transmitted(),
            );

            for hourly_sample in analytics.get_hourly_samples() {
                database.store_hourly_sample(hourly_sample);
            }
        }

        interval.tick().await;
    }
}
