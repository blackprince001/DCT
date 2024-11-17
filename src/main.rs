use std::error::Error;

use sysinfo::Networks;

mod network;

fn main() -> Result<(), Box<dyn Error>> {
    let networks = Networks::new_with_refreshed_list();
    let interface = String::from("en0");
    let scrape_time = 1000; // 1 second
    let packetsize = network::types::DataSize::Kilobyte;

    network::net::run(networks, &interface, scrape_time, packetsize)?;
    Ok(())
}
