extern crate yaml_rust;

use yaml_rust::{YamlLoader};
use anyhow::{Result, Context};
use std::fs;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct Env {
    pub my_ip_address: Ipv4Addr,
    pub my_port: u16,
    pub maximum_port: u16,
}
impl Env {
    pub fn new() -> Self {
        let (my_ip_address, my_port, maximum_port) = get_env()
            .unwrap_or_else(|e| panic!(format!("{}", e)));
        Env {
            my_ip_address,
            my_port,
            maximum_port
        }
    }
}

fn get_from_file() -> Result<String, anyhow::Error> {
    let contents = fs::read_to_string("env.yml")
        .with_context(|| "No file with that name exists")?;
    Ok(contents)
}

fn get_env() -> anyhow::Result<(Ipv4Addr, u16, u16)> {
    let source = &get_from_file()? as &str;
    let docs = YamlLoader::load_from_str(source)?;
    let doc = &docs[0];
    let my_ip_address_str = doc["my_ip_address"].as_str()
        .unwrap_or_else(|| panic!("could not the element 'my_ip_address'"))
        .to_string();
    let my_ip_address =
        Ipv4Addr::from_str(my_ip_address_str.as_str())
            .unwrap_or_else(|_| panic!("The argument given cannot be translated into an ipv4 address"));
    let my_port = doc["my_port"].as_i64()
        .unwrap_or_else(|| panic!("could not the element 'my_port'"))
        as u16;
    let maximum_port = doc["maximum_port_num"].as_i64()
        .unwrap_or_else(|| panic!("could not the element 'maximum_port_num'"))
        as u16;
    Ok((my_ip_address, my_port, maximum_port))
}