use std::{
    error::Error,
    fmt, thread,
    time::{self},
};
use sysinfo::Networks;

const KILOBYTE: u64 = 1024;

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

pub fn run(mut netd: Networks, interface: &String, scrape_time: u64) -> Result<(), InterfaceError> {
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

    loop {
        thread::sleep(time::Duration::from_millis(scrape_time));
        netd.refresh();

        if let Some(network_data) = netd.get(interface) {
            println!(
                "Data Usage - Transmitted: {} KB, Received: {} KB",
                network_data.transmitted() / KILOBYTE,
                network_data.received() / KILOBYTE
            );
        } else {
            return Err(InterfaceError {
                message: format!("Lost connection to interface '{}'", interface),
            });
        }
    }
}
