use pnet::transport::TransportSender;
use crate::packet::data::PacketInfo;
use crate::packet::write::{build_packet, rewrite_destination_port};
use pnet::packet::tcp::MutableTcpPacket;
use anyhow::anyhow;
use std::thread;
use std::time::Duration;
use std::net::IpAddr;

pub fn send_packet(
    sender: &mut TransportSender,
    packet_info: &PacketInfo,
) -> anyhow::Result<()> {
    let mut packet = build_packet(packet_info);

    // 1~env.ymlで指定されたポートまで順番にパケットを送りつける
    for port in 1..=packet_info.maximum_port {
        let mut tcp_header = MutableTcpPacket::new(&mut packet)
            .ok_or_else(|| anyhow!("invalid packet"))?;
        rewrite_destination_port(port, &mut tcp_header, packet_info);

        // スリープしないと早すぎてうまくいかない
        thread::sleep(Duration::from_millis(5));
        sender.send_to(tcp_header, IpAddr::V4(packet_info.target_ip_address))?;
    }
    Ok(())
}
