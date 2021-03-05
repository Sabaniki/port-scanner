use log::error;
use std::process::exit;

pub fn print_and_exit(message: &str) -> ! {
    error!("{}", message);
    exit(-1);
}