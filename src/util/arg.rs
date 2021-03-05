use clap::{ArgMatches, App, Arg, ArgGroup};
use crate::packet;
use std::net::Ipv4Addr;
use std::str::FromStr;
use crate::util::error::print_and_exit;
use std::process::exit;
use log::error;

pub struct Args {
    pub target_ip_address: Ipv4Addr,
    pub scan_type: packet::ScanType,
}

impl Args {
    pub fn new() -> Self {
        let app = create_arg();
        let target_ip_address = get_target_ip_address(&app);
        let scan_type = get_scan_type(&app);
        Args {
            target_ip_address,
            scan_type,
        }
    }
}

fn create_arg() -> ArgMatches {
    App::new("simple port scan application")
        .version("1.0.0")
        .author("sabaniki")
        .about("this is simple port scan application written in Rust.")
        .arg(
            Arg::new("target_ip_address")
                .about("specify target_ip_address")
                .value_name("target for port scan")
                .index(1)
                .required(true)
        )
        .arg(Arg::new("syn").index(2))
        .arg(Arg::new("fin").index(3))
        .arg(Arg::new("xmas").index(4))
        .arg(Arg::new("null").index(5))
        .group(
            ArgGroup::new("scan_type")
                .args(&["syn", "fin", "xmas", "null"])
                .required(true)
        )
        .get_matches()
}

fn get_target_ip_address(arg_matches: &ArgMatches) -> Ipv4Addr {
    let raw = arg_matches.value_of("target_ip_address")
        .unwrap_or_else(||
            panic!("could not get the arg [target_ip_address]")
        );
    let parsed = Ipv4Addr::from_str(raw)
        .unwrap_or_else(|_|
            print_and_exit(
                format!(
                    "given argument [{}] is invalid ipv4 address",
                    raw
                ).as_str()
            )
        );
    parsed
}

fn get_scan_type(arg_matches: &ArgMatches) -> packet::ScanType {
    let method_name = arg_matches.value_of("scan_type").unwrap_or_else(||
        panic!("could not get the arg [scan_type]")
    );
    match method_name {
        "syn" => packet::ScanType::Syn,
        "fin" => packet::ScanType::Fin,
        "xmas" => packet::ScanType::Xmas,
        "null" => packet::ScanType::Null,
        _ => print_and_exit(
            format!("given argument [{}] is invalid scan type", method_name).as_str()
        )
    }
}
