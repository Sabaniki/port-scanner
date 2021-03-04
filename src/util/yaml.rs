extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use anyhow::{Result, Context};
use std::fs;

fn get_from_file() -> Result<String, anyhow::Error> {
    let contents = fs::read_to_string("env.yml")
        .with_context(||"No file with that name exists")?;
    Ok(contents)
}

pub fn get_env() -> anyhow::Result<(String, u16, u16)> {
    let source = &get_from_file()? as &str;
    let docs = YamlLoader::load_from_str(source)?;
    let doc = &docs[0];
    let my_ip_address = doc["my_ip_address"].as_str()
        .unwrap_or_else(|| panic!("could not the element 'my_ip_address'"))
        .to_string();
    let my_port = doc["my_port"].as_i64()
        .unwrap_or_else(||panic!("could not the element 'my_port'"))
        as u16;
    let maximum_port_num = doc["maximum_port_num"].as_i64()
        .unwrap_or_else(|| panic!("could not the element 'maximum_port_num'"))
        as u16;
    Ok((my_ip_address, my_port, maximum_port_num))
}