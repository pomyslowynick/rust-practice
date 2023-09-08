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
        let mut buffer = vec![0; 1500];
        let ip_packet = Ipv4Header::new(0, 64, 6, self.ip_header.destination(), self.ip_header.source());


        let tcp_packet = TcpHeader::new(self.tcp_header.destination_port(), self.tcp_header.destination_port(), 0, 1 );

        use std::io::Write;
        let mut unwritten = &mut buffer[..];
        let writing_result = ip_packet.write(&mut unwritten).unwrap();
        let tcp_result = tcp_packet.write(&mut unwritten).unwrap();

        let unwritten = unwritten.len();
        println!("{:?}", buffer);
        nic.send(&buffer[..buffer.len()-unwritten]);
    }
}
