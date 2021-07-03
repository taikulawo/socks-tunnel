use std::{io::{self, ErrorKind}, net::{IpAddr, Ipv4Addr, SocketAddr}};
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use log::{
    debug
};

use crate::core::config::ServerConfig;
pub struct Server {

}

impl Server {
    pub fn bind(&self) -> io::Result<TcpListener> {
        let socket = TcpSocket::new_v4()?;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 127, 127, 127)), 1010);
        match socket.bind(addr) {
            Ok(..) => {},
            Err(ref err) if err.kind() == ErrorKind::AddrInUse => {
                debug!("already be used");
            },
            Err(err) => return Err(err)
        }
        let listener = socket.listen(1024)?;
        return Ok(listener);
    }

    pub async fn run_server(self) -> io::Result<()>{
        let listener = self.bind()?;
        loop {
            let (connection, socket_addr) = listener.accept().await?;
            tokio::spawn(async move {
                connection;
            });
        }
    }
}

pub struct TcpServerConnectionProcessor {
    stream: TcpStream,
    config: ServerConfig
}

impl TcpServerConnectionProcessor {
    pub fn new(stream: TcpStream, config: ServerConfig) -> Self {
        TcpServerConnectionProcessor {
            stream,
            config
        }
    }
    pub async fn serve(self) -> io::Result<()>{
        todo!()
    }
}