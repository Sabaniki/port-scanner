mod util;

#[macro_use]
extern crate log;

use std::env;
use crate::util::packet::PacketInfo;
use crate::util::arg::Args;
use crate::util::env::Env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = Args::new();
    let env = Env::new();
    let packet_info = PacketInfo {
        my_ip_address: env.my_ip_address,
        target_ip_address: args.target_ip_address,
        my_port: env.my_port,
        maximum_port: env.maximum_port,
        scan_type: args.scan_type
    };
    println!("{}", packet_info.target_ip_address);
}
