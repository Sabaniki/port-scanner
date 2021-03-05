mod util;
mod packet;

#[macro_use]
extern crate log;

use std::env;
use packet::PacketInfo;
use crate::util::arg::Args;
use crate::util::env::Env;
use pnet::transport::{
    self,
    TransportChannelType,
    TransportProtocol,
    TransportReceiver,
    TransportSender,
};
use pnet::packet::ip::IpNextHeaderProtocols;
use crate::util::error::print_and_exit;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

fn main() {
    env::set_var("RUST_LOG", "debug");
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record|
            writeln!(buf, "\x1b[{}m{}\x1b[m: {}", 31, record.level(), record.args())
        )
        .init();

    let args = Args::new();
    let env = Env::new();
    let packet_info = PacketInfo {
        my_ip_address: env.my_ip_address,
        target_ip_address: args.target_ip_address,
        my_port: env.my_port,
        maximum_port: env.maximum_port,
        scan_type: args.scan_type,
    };

    // トランスポート層のチャンネルを開く(内部的にはソケット)
    let (mut sender, mut receiver) = transport::transport_channel(
        1024,
        TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp)),
    ).unwrap_or_else(|e| print_and_exit(format!("{}", e).as_str()));

    // パケットの送信と受信を並列に行う
    // rayon::join(
    //     // TODO: send_packet(&mut sender, &packet_info),
    //     // TODO: receive_packets(&mut receive, &packet_info),
    // );
}
