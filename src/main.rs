mod util;

#[macro_use]
extern crate log;

use std::env;





fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _arg = util::app::get_scan_type();
}
