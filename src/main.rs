mod util;

#[macro_use]
extern crate log;

use std::env;
use util::app::get_scan_type;
use std::process::exit;
use anyhow::Error;
use crate::util::packet::ScanType;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let arg = util::app::get_scan_type();
}
