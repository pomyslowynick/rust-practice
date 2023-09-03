use tun_tap;
use etherparse;

fn main() {
    let iface = tun_tap::Iface::without_packet_info("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1504];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        println!("{:x?}", &buffer);


        //let tcp_header = etherparse::TcpHeaderSlice::from_slice(&buffer).unwrap();
        match etherparse::PacketHeaders::from_ethernet_slice(&buffer) {
            Err(value) => println!("{:?}", value),
            Ok(value) => {
                println!("link: {:?}", value.link);
                println!("vlan: {:?}", value.vlan);
                println!("ip: {:?}", value.ip);
                println!("transport: {:?}", value.transport);
            }

        }
    }
}

fn connect() {
    unimplemented!()
}
