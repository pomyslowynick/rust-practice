use etherparse::*;
use tun_tap::Iface;
// Connection states
enum TCPState {}

// Connection struct
pub struct Connection<'a> {
    ip_header: Ipv4HeaderSlice<'a>,
    tcp_header: TcpHeaderSlice<'a>,
}

impl <'a> Connection<'a> {
    pub fn new(
        ip_header: Ipv4HeaderSlice<'a>,
        tcp_header: TcpHeaderSlice<'a>,
    ) -> Self {
        Self {
            ip_header,
            tcp_header,
        }
    }

    pub fn connect(&self, nic: &Iface) {
        println!("nic name {:?}", nic.name());
    }
}
