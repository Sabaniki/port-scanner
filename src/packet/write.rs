use crate::packet::data::PacketInfo;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};

const TCP_SIZE: usize = 20;

// パケットを作成する
pub fn build_packet(packet_info: &PacketInfo) -> [u8; TCP_SIZE] {
    // 0u8: 符号なし8bit整数としての0
    let mut tcp_buffer = [0u8; TCP_SIZE];
    let mut tcp_header = MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();
    tcp_header.set_source(packet_info.my_port);

    // オプションを含まないため20オクテットまでがTCPヘッダ。
    // 4オクテット単位で指定するため引数は5。
    // オプションやコントロールフラグについて忘れたら
    // https://www.infraexpert.com/study/tcpip8.html を参照
    tcp_header.set_data_offset(5);
    tcp_header.set_flags(packet_info.scan_type as u16);
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ip_address,
        &packet_info.target_ip_address,
    );
    tcp_header.set_checksum(checksum);

    tcp_buffer
}

