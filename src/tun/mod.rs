mod netstack;
use std::{fmt::Write, os::unix::prelude::AsRawFd};
use std::io::Read;

use log::debug;
use pretty_hex::{pretty_hex, simple_hex, PrettyHex};
use smoltcp::{
    iface::EthernetInterfaceBuilder,
    phy::{TapInterface, wait as phy_wait},
    socket::{AnySocket, SocketSet, TcpSocket, TcpSocketBuffer},
    time::Instant,
    wire::{Ipv4Packet, TcpPacket},
};
const TUN_NAME: &str = "utun666";
macro_rules! must {
    ($expr: expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => {
                log::trace!("{}", err);
                return;
            }
        }
    };
}
pub fn run_tun() {
    let mut config = tun::Configuration::default();
    config
        .name(TUN_NAME)
        .address((10, 0, 0, 1))
        .netmask((255, 255, 255, 0))
        .up();
    config.platform(|config| {
        config.packet_information(false);
    });
    let mut dev = tun::create(&config).unwrap();
    let mut buf = vec![0; 2000];

    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);
    let raw_socket = TapInterface::new(TUN_NAME).unwrap();
    let fd = raw_socket.as_raw_fd();
    let mut builder = EthernetInterfaceBuilder::new(raw_socket);

    let mut iface = builder.finalize();
    loop {
        let timestamp = Instant::now();
        match iface.poll(&mut sockets, timestamp) {
            Ok(_) => {}
            Err(err) => {
                debug!("poll error: {}", err);
            }
        }
        {
            let mut socket = sockets.get::<TcpSocket>(tcp_handle);
            if !socket.is_open() {
                socket.listen(9999).unwrap();
            }
            if socket.can_send() {
                writeln!(socket, "hello").unwrap();
            }
            if socket.can_recv() {
                socket.rec
                pretty_hex()
            }
        }
        // let amount = dev.read(&mut buf).unwrap();
        // let p = &mut buf[..amount];
        // println!("{}", pretty_hex(&p));
        // let mut packet = match Ipv4Packet::new_checked(&mut buf[..amount]) {
        //     Err(err) => {
        //         println!("{}", err);
        //         continue;
        //     }
        //     Ok(x) => x,
        // };
        // let payload = packet.payload_mut();
        // // for mut real_tcp_socket in sockets.iter_mut().filter_map(TcpSocket::downcast) {
        // //     real_tcp_socket.accepts()
        // // }
        // let tcp = match TcpPacket::new_checked(payload) {
        //     Ok(t) => t,
        //     Err(err) => {
        //         eprintln!("{}", err);
        //         continue;
        //     },
        // };

        // let src = tcp.src_port();
        // println!("src port {}", src);
        // println!("{:?}", &buf[0 .. amount]);
        phy_wait(fd, iface.poll_delay(&sockets, timestamp)).expect("wait error");
    }
}

#[test]
fn run() {
    run_tun();
}
