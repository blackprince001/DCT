mod network;

use std::error::Error;

use network::run;
use sysinfo::Networks;

fn main() -> Result<(), Box<dyn Error>> {
    let networks = Networks::new_with_refreshed_list();
    let interface = String::from("en0");
    let scrape_time = 1000; // 1 second

    run(networks, &interface, scrape_time)?;
    Ok(())
}
