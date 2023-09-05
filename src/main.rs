use tun_tap;
use etherparse;

fn main() {
    let iface = tun_tap::Iface::without_packet_info("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1500];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        println!("{:x?}", &buffer);


        //let tcp_header = etherparse::TcpHeaderSlice::from_slice(&buffer).unwrap();
        match etherparse::Ipv4HeaderSlice::from_slice(&buffer) {
            Err(value) => println!("{:?}", value),
            Ok(value) => {
                println!("Ipv4 Header: {:?}", value);
                println!("Ipv4 Total Length: {:?}", value.total_len());
                println!("Ipv4 source: {:?}", value.source());
                println!("Ipv4 protocol: {:?}", value.protocol());
                println!("Ipv4Header 2 {:?}", value.to_header());
                println!("Ipv4Header IHL {:?}", value.ihl());

                match etherparse::TcpHeaderSlice::from_slice(&buffer[(value.ihl() * 4) as usize..]) {
                    Err(tcpheader) => println!("{:?}", tcpheader),
                    Ok(tcpheader) => println!("{:?}", tcpheader.to_header())
                }
            }
        }
    }
}

fn connect() {
    unimplemented!()
}
