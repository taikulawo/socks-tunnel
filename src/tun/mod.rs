use std::io::Read;

use smoltcp::wire::Ipv4Packet;

pub fn run_tun() {
    let mut config = tun::Configuration::default();
    config.address((10, 0, 0, 1)).netmask((255, 255, 255, 0)).up();
    config.platform(|config| {
        config.packet_information(true);
    });
    let mut dev = tun::create(&config).unwrap();
    let mut buf = vec![0; 2000];
    loop {
        let amount = dev.read(&mut buf).unwrap();
        let packet = match Ipv4Packet::new_checked(&mut buf[..amount]) {
            Err(err) => {
                println!("{}", err);
                continue;
            },
            Ok(x ) => x
        };
        println!("{}", packet.dst_addr());
        // println!("{:?}", &buf[0 .. amount]);
    }
}

#[test]
fn run() {
    run_tun();
}