use std::net::Ipv4Addr;
use pnet::packet::tcp::{TcpFlags};

pub struct PacketInfo {
    pub my_ip_address: Ipv4Addr,
    pub target_ip_address: Ipv4Addr,
    pub my_port: u16,
    pub maximum_port: u16,
    pub scan_type: ScanType,
}

// Copyが実装されているので、代入などの際にデフォルトでコピーが生成される
// enumは要素にpubいらない
#[derive(Copy, Clone)]
pub enum ScanType {
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmas = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
    Null = 0,
}
