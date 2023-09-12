use etherparse;
use tun_tap;
mod tcp;

fn main() {
    let iface = tun_tap::Iface::without_packet_info("tun1", tun_tap::Mode::Tun)
        .expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1500];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        match etherparse::Ipv4HeaderSlice::from_slice(&buffer) {
            Err(ip_header) => println!("Error parsing IPv4 {:?}", ip_header),
            Ok(ip_header) => {
                if ip_header.protocol() == 6 {
                    match etherparse::TcpHeaderSlice::from_slice(
                        &buffer[(ip_header.ihl() * 4) as usize..],
                    ) {
                        Err(tcp_header) => println!("{:?}", tcp_header),
                        Ok(tcp_header) => {
                            let connection = tcp::Connection::new(
                                ip_header,
                                tcp_header,
                            );
                            connection.connect(&iface);
                        }
                    }
                }
            }
        }
    }
}
