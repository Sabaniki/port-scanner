use std::net::Ipv4Addr;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};

pub struct PacketInfo {
    my_ip_address: Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType
}

#[derive(Copy, Clone)]
pub enum ScanType{
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmas = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
    Null = 0
}
