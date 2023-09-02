use tun_tap;
use etherparse;

fn main() {
    let iface = tun_tap::Iface::without_packet_info("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1504];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        println!("{:x?}", &buffer[0..50]);
    }
}

fn connect() {
    unimplemented!()
}
