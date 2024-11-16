use std::{thread, time};
use sysinfo::Networks;

pub fn run(mut netd: Networks) {
    // TODO
    // Take an interface
    // refresh interface scrape over a loop
    // print the results over stdin after a time period - tick

    let interfaces = Networks::list(&netd);

    println!("Network Interfaces: {:?}", interfaces.keys());

    thread::sleep(time::Duration::from_millis(10000));

    netd.refresh();

    for (interface_name, network) in &netd {
        println!("in {interface_name}: {} kB", network.received() / 1024);
    }
}
