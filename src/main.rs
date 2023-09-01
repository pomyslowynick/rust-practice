use tun_tap;

fn main() {
    println!("Hello, world!");
    let iface = tun_tap::Iface::new("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    println!("Hello, world!");
    let mut buffer = vec![0; 1504];
    loop {
        let received = iface.recv(&mut buffer).unwrap();
        println!("{}", received);
        println!("{:x?}", &buffer[0:50]);
    }
}

fn connnect() {
    unimplemented!()
}
