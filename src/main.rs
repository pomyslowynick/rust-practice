use tun_tap;

fn main() {
    println!("Hello, world!");
    let iface = tun_tap::Iface::new("tun1", tun_tap::Mode::Tun).expect("Failed to create a tun interface");
    let mut buffer = vec![0; 1504];
    loop {
        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(10);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
        let received = iface.recv(&mut buffer).unwrap();
        println!("{}", received);
        println!("{:?}", &buffer);
    }
}

fn connnect() {
    unimplemented!()
}
