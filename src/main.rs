mod util;
mod packet;

#[macro_use]
extern crate log;

fn main() {
    util::app::run().unwrap_or_else(|e|error!("{}", e))
}
