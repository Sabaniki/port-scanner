// mod util;
// mod packet;



use std::env;
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
use crate::util::logger;
use crate::packet::data::PacketInfo;

pub fn run() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");

    let mut logger = logger::create_colored_logger();
    logger.init();

    let args = Args::new()?;
    let env = Env::new()?;
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
    )?;
    Ok(())

    // パケットの送信と受信を並列に行う
    // rayon::join(
    //     // TODO: send_packet(&mut sender, &packet_info),
    //     // TODO: receive_packets(&mut receive, &packet_info),
    // );
}