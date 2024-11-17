use std::{
    error::Error,
    fmt, thread,
    time::{self},
};
use sysinfo::Networks;

use super::types::DataSize;

#[derive(Debug)]
pub struct InterfaceError {
    message: String,
}

impl fmt::Display for InterfaceError {
    // this was totally unnecessary but solved a lot
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InterfaceError {}

pub fn run(
    mut netd: Networks,
    interface: &String,
    scrape_time: u64,
    data_size: DataSize,
) -> Result<(), InterfaceError> {
    let interfaces: Vec<&String> = Networks::list(&netd).keys().collect();

    if !interfaces.contains(&interface) {
        return Err(InterfaceError {
            message: format!(
                "Interface '{}' not found. Available interfaces: {}",
                interface,
                interfaces
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        });
    }

    println!("Monitoring network interface: {}", interface);
    println!("Sampling every {} milliseconds", scrape_time);

    let size = data_size.size_to_value();

    loop {
        thread::sleep(time::Duration::from_millis(scrape_time));
        netd.refresh();

        if let Some(network_data) = netd.get(interface) {
            println!(
                "Data Usage - Transmitted: {}, Received: {} ({})",
                network_data.transmitted() as f64 / size as f64,
                network_data.received() as f64 / size as f64,
                data_size.str()
            );
        } else {
            return Err(InterfaceError {
                message: format!("Lost connection to interface '{}'", interface),
            });
        }
    }
}
