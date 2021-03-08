use crate::packet::data::PacketInfo;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};
use pnet::packet::MutablePacket;

const TCP_SIZE: usize = 20;

// パケットを作成する
pub fn build_packet(packet_info: &PacketInfo) -> [u8; TCP_SIZE] {
    // 0u8: 符号なし8bit整数としての0
    let mut tcp_buffer = [0u8; TCP_SIZE];
    let mut tcp_header = MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();
    tcp_header.set_source(packet_info.my_port);

    // オプションを含まないため20オクテットまでがTCPヘッダ。
    // 4オクテット単位で指定するためオフセットに与える引数は5。
    // オプションやコントロールフラグについて忘れたら、
    // https://www.infraexpert.com/study/tcpip8.html を参照。
    tcp_header.set_data_offset(5);
    // ちなみにPacketInfoがCopyトレイトを実装していないとここでCannot moveエラーが起きる
    tcp_header.set_flags(packet_info.scan_type as u16);

    // チェックサムを計算して設定
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ip_address,
        &packet_info.target_ip_address,
    );
    tcp_header.set_checksum(checksum);

    tcp_buffer
}

// TCPヘッダの宛先ポート情報を書き換える。
pub fn rewrite_destination_port(
    target: u16,
    tcp_header: &mut MutableTcpPacket,
    packet_info: &PacketInfo,
) {
    tcp_header.set_destination(target);

    // TCPヘッダの中身をいじったので、
    // チェックサムを計算し直さなければならない。
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ip_address,
        &packet_info.target_ip_address,
    );
    tcp_header.set_checksum(checksum);
}