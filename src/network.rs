use std::{thread, time};
use sysinfo::Networks;

pub fn run() {
    // TODO
    // Take an interface
    // refresh interface scrape over a loop
    // print the results over stdin after a time period - tick

    let mut networks = Networks::new_with_refreshed_list();
    // Waiting a bit to get data from network...
    thread::sleep(time::Duration::from_millis(10000));
    // Refreshing again to generate diff.
    networks.refresh();

    for (interface_name, network) in &networks {
        println!("in {interface_name}: {} kB", network.received() / 1024);
    }
}
