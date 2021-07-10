use std::{io::{self, ErrorKind}, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc, time::Duration};
use futures::future::{self, Either};
use tokio::{io::copy, net::{TcpListener, TcpSocket, TcpStream}, time::timeout};
use log::{
    debug,
    error
};

use crate::core::{Context, config::ServerConfig, copy::pipe, socks5::Socks5Address};
pub struct Server {
    context: Arc<Context>,
    config: ServerConfig
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
            let handler = TcpServerConnectionHandler {
                config: self.config.clone(),
                context:self.context.clone(),
                stream: connection,
                remote_addr_in_local: socket_addr
            };
            tokio::spawn(async move {
                handler.serve()
            });
        }
    }
}

pub struct TcpServerConnectionHandler {
    stream: TcpStream,
    config: ServerConfig,
    context: Arc<Context>,
    remote_addr_in_local: SocketAddr
}

impl TcpServerConnectionHandler {
    pub async fn serve(mut self) -> io::Result<()>{
        let address = match Socks5Address::from_addr(&mut self.stream).await {
            Ok(addr) => addr,
            Err(err) => {
                error!("error when socks handshakeing {}", err);
                return Ok(())
            },
        };
        // 开始连接 remote
        let mut remote_stream = match timeout(
            Duration::new(5, 0), 
            TcpServerConnectionHandler::connect_remote(self.context.as_ref(), &address)
        ).await {
            Ok(stream_result) => match stream_result{
                Ok(stream) => stream,
                Err(err) => {
                    error!("failed to connect to remote {} with error {}", address, err);
                    return Ok(())
                },
            },
            Err(err) => {
                error!("timeout at {} with error {}", address, err);
                return Ok(())
            }
        };
        let (mut reader, mut writer) = remote_stream.into_split();
        let (mut local_reader, mut local_writer) = self.stream.into_split();
        // 将两个 connection reader， writer对接
        let fut1 = pipe(&mut reader, &mut local_writer);
        let fut2 = pipe(&mut local_reader, &mut writer);
        match future::select(fut1, fut2).await {
            Either::Left((Ok(..), ..)) => {
                
            },
            Either::Left((Err(err), ..)) => {
                debug!("connection error on {} to {} with error {}", &address, &self.remote_addr_in_local, err);
            }
            Either::Right((Ok(..), ..)) => {

            },
            Either::Right((Err(err), ..)) => {
                debug!("connection error on {} to {} with error {}",&self.remote_addr_in_local, &address, err);
            }
        }
        Ok(())
    }

    pub async fn connect_remote(ctx: &Context, address: &Socks5Address) -> io::Result<TcpStream> {
        let addr = match *address {
            Socks5Address::Domain(ref domain, ref port) => {
                Self::lookup_domain(&domain).await?
            },
            Socks5Address::Ipv4Addr(ref ipv4) => SocketAddr::V4(ipv4.to_owned()),
            Socks5Address::Ipv6Addr(ref ipv6) => SocketAddr::V6(ipv6.to_owned())
        };
        TcpStream::connect(addr).await
    }

    pub async fn lookup_domain(name: &str) -> io::Result<SocketAddr> {
        unimplemented!()
    }
}