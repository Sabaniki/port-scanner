extern crate yaml_rust;

use yaml_rust::{YamlLoader};
use anyhow::{Result, Context};
use std::fs;
use std::net::Ipv4Addr;
use std::str::FromStr;
use log::error;
use std::process::exit;
use anyhow::anyhow;

pub struct Env {
    pub my_ip_address: Ipv4Addr,
    pub my_port: u16,
    pub maximum_port: u16,
}

impl Env {
    pub fn new() -> anyhow::Result<Self> {
        let (my_ip_address, my_port, maximum_port) =
            get_env()?;
        let env = Env {
            my_ip_address,
            my_port,
            maximum_port,
        };
        Ok(env)
    }
}

fn get_from_file() -> Result<String, anyhow::Error> {
    let contents = fs::read_to_string("env.yml")
        .with_context(|| "No file with that name exists")?;
    Ok(contents)
}

fn get_env() -> anyhow::Result<(Ipv4Addr, u16, u16)> {
    // ファイルの中身取ってきてyamlに変換
    // get_from_fileに&が必要な理由がよくわからない(コンパイラに従った)
    let source = &get_from_file()? as &str;
    let docs = YamlLoader::load_from_str(source)?;
    let doc = &docs[0];

    // 丁寧にyamlから環境変数取ってくる
    let my_ip_address_str = doc["my_ip_address"].as_str()
        .ok_or_else(|| anyhow!("'my_ip_address' is not defined in env.yml"))?
        .to_string();
    // そのうちipv6にも対応させたい気持ちあり
    let my_ip_address =
        Ipv4Addr::from_str(my_ip_address_str.as_str())
            .with_context(|| format!("in env.yml, [{}] is invalid ipv4 address", my_ip_address_str))?;
    let my_port = doc["my_port"].as_i64()
        .ok_or_else(|| anyhow!("'my_port' is not defined in env.yml"))?
        as u16;
    let maximum_port = doc["maximum_port_num"].as_i64()
        .ok_or_else(|| anyhow!("'maximum_port_num' is not defined in env.yml"))?
        as u16;
    Ok((my_ip_address, my_port, maximum_port))
}