mod network;

use network::run;
use sysinfo::Networks;

fn main() {
    let some_network = Networks::new_with_refreshed_list();
    run(some_network);
}
