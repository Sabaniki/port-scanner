mod util;

#[macro_use]
extern crate log;

use std::env;
use crate::util::yaml::get_env;


fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _arg = util::arg::get_scan_type();
    let (my_ip_address, my_port, maximum_port_num) = get_env()
        .unwrap_or_else(|_|panic!("error!"));
    println!("{}", my_ip_address);
    println!("{}", my_port);
    println!("{}", maximum_port_num);
}
