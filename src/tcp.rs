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
        let mut payload = vec![0; 0];
        let mut ip_packet = Ipv4Header::new(0, 64, 6, self.ip_header.destination(), self.ip_header.source());

        let mut tcp_packet = TcpHeader::new(self.tcp_header.destination_port(), self.tcp_header.source_port(), self.tcp_header.sequence_number() + 1, 100 );
        tcp_packet.checksum = tcp_packet.calc_checksum_ipv4(&ip_packet, &payload).unwrap();
        
        ip_packet.set_payload_len(tcp_packet.header_len() as usize);

        use std::io::Write;
        let mut unwritten = &mut buffer[..];
        let ip_result = ip_packet.write(&mut unwritten).unwrap();
        let tcp_result = tcp_packet.write(&mut unwritten).unwrap();
        unwritten.write(&payload);
        let unwritten = unwritten.len();
        println!("Buffer after writing {:x?}", &buffer[..buffer.len()-unwritten]);

        nic.send(&buffer[..buffer.len()-unwritten]);
    }
}
