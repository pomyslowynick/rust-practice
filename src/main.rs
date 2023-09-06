use tun_tap;
use etherparse;

fn main() {
    let iface = tun_tap::Iface::without_packet_info("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1500];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        match etherparse::Ipv4HeaderSlice::from_slice(&buffer) {
            Err(value) => println!("Error parsing IPv4 {:?}", value),
            Ok(value) => {
                if value.protocol() == 6 {
                    match etherparse::TcpHeaderSlice::from_slice(&buffer[(value.ihl() * 4) as usize..]) {
                        Err(tcpheader) => println!("{:?}", tcpheader),
                        Ok(tcpheader) => {
                            println!("{:?}", tcpheader.to_header());
                            connect(tcpheader);
                        }
                    }
                }
            }
        }
    }
}
