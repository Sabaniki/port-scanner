use clap::{ArgMatches, App, Arg, ArgGroup};
use super::packet;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};
use log;

fn create_arg() -> ArgMatches {
    App::new("simple port scan application")
        .version("1.0.0")
        .author("sabaniki")
        .about("this is simple port scan application written in Rust.")
        .arg(Arg::new("syn"))
        .arg(Arg::new("fin"))
        .arg(Arg::new("xmas"))
        .arg(Arg::new("null"))
        .group(
            ArgGroup::new("scan_type")
                .args(&["syn", "fin", "xmas", "null"])
                .required(true)
        )
        .get_matches()
}

pub fn get_scan_type() -> packet::ScanType {
    let app = create_arg();
    let method_name = app.value_of("scan_type").unwrap_or_else(||
        panic!("could not get the arg [scan_type]")
    );
    match method_name {
        "syn" => packet::ScanType::Syn,
        "fin" => packet::ScanType::Fin,
        "xmas" => packet::ScanType::Xmas,
        "null" => packet::ScanType::Null,
        _ => panic!("cannot opt for such a method.")
    }
}
