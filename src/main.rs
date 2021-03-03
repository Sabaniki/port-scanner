mod util;

#[macro_use]
extern crate log;

use std::env;
use util::app::get_arg;
use std::process::exit;
use anyhow::Error;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let arg = util::app::get_arg();
    match arg {
        Ok(m) => {println!("{}", m)},
        Err(e) => {println!("{}", e); exit(-1);}
    }
}
