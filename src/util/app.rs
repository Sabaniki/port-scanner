use clap::{ArgMatches, App, Arg, ArgGroup};
use anyhow::Context;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};



fn create_app() -> ArgMatches {
    App::new("simple port scan application")
        .version("1.0.0")
        .author("sabaniki")
        .about("this is simple port scan application written in Rust.")
        .arg(Arg::new("syn"))
        .arg(Arg::new("fin"))
        .arg(Arg::new("xmas"))
        .arg(Arg::new("null"))
        .group(
            ArgGroup::new("methods")
                .args(&["syn", "fin", "xmas", "null"])
                .required(true)
        )
        .get_matches()
}

pub fn get_arg() -> Result<String, anyhow::Error> {
    let app = create_app();
    let method_name = app.value_of("methods")
        .with_context(||"could not get the arg [methods]")?;
    Ok(method_name.to_string())
    // let interface_name = app.value_of("interface_name")
    //     .with_context(||"could not get the arg [interface_name]")?;
    // Ok(interface_name.to_string())
}
