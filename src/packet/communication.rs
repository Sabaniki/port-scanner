use pnet::transport::TransportSender;
use crate::packet::data::PacketInfo;
use crate::packet::write::build_packet;
use pnet::packet::tcp::MutableTcpPacket;
use anyhow::anyhow;

pub fn send_packet(
    sender: &mut TransportSender,
    packet_info: &PacketInfo,
) -> anyhow::Result<()> {
    let mut packet = build_packet(packet_info);
    for port in 1..=packet_info.maximum_port {
        let mut tcp_header = MutableTcpPacket::new(&mut packet).ok_or_else(|| anyhow!("invalid packet"))?;
    }
    Ok(())
}