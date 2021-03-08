mod util;
mod packet;

#[macro_use]
extern crate log;

fn main() {
    // 内部で発生した様々なエラーは最終的にここにたどり着く…
    util::app::run().unwrap_or_else(|e| error!("{}", e))
}
