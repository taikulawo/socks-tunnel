use core::slice;
use std::{error::Error, net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6}, u16, u8, usize, vec};

use futures::io;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

/// socks5 implementation

pub const SOCKS5_CMD_COMMECT: u8 = 0x01;
pub const SOCKS5_CMD_BIND: u8 = 0x02;
pub const SOCKS5_CMD_UDP: u8 = 0x03;
pub const SOCKS5_ATYP_IPV4: u8 = 0x01;
pub const SOCKS5_ATYP_DOMAIN: u8 = 0x03;
pub const SOCKS5_ATYP_IPV6: u8 = 0x04;
pub struct Socks5Server {
    reader: TcpStream,
}

impl Socks5Server {
    pub async fn new(stream: TcpStream) -> Socks5Server {
        Socks5Server {
            reader: stream
        }
    }

    pub async fn read_addr() -> Socks5Address {
        todo!()
    }
}

pub enum Socks5Address {
    Domain(String, u16),
    Ipv4Addr(SocketAddrV4),
    Ipv6Addr(SocketAddrV6)
}

/// https://www.ietf.org/rfc/rfc1928.txt
impl Socks5Address {
    pub async fn from_addr(reader: &mut TcpStream) -> Result<Socks5Address, Socks5Error> {
        let mut buf = [0u8; 3];
        reader.read_exact(&mut buf).await?;
        let version = &buf[0];
        if *version != 0x05 {
            return Err(Socks5Error::UnsupportedSocks5Version(*version));
        }
        let method = &buf[1];
        if *method != 0x00 {
            return Err(Socks5Error::UnsupportedAuthorizationMethod(*method));
        }
        reader.write(&[0x05, 0x00]).await?;
        // todo!();
        let command = Socks5Address::parse_request(reader).await?;
        let address= Socks5Address::parse_address(reader).await?;
        Ok(address)
    }

    async fn parse_request(reader: &mut TcpStream) -> Result<Socks5Command, Socks5Error>{
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let version = &buf[0];
        if *version != 0x05 {
            return Err(Socks5Error::UnsupportedSocks5Version(*version));
        }
        let cmd = match buf[1] {
            SOCKS5_CMD_COMMECT => Socks5Command::CONNECT,
            SOCKS5_CMD_BIND => Socks5Command::BIND,
            SOCKS5_CMD_UDP => Socks5Command::UDP,
            _ => return Err(Socks5Error::UnsupportedCommand(buf[1]))
        };
        Ok(cmd)
    }
    async fn parse_address(r: &mut TcpStream) -> Result<Socks5Address, Socks5Error>{
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf).await?;
        let addr = match buf[0] {
            SOCKS5_ATYP_IPV4 => {
                let mut buf = [0u8; 6];
                r.read_exact(&mut buf).await?;
                let ipv4 = Ipv4Addr::new(buf[0], buf[1], buf[2],buf[3]);
                let port = unsafe {
                    let raw_port = &buf[4..];
                    u16::from_be(*(raw_port as *const _ as *const _))
                };
                Socks5Address::Ipv4Addr(SocketAddrV4::new(ipv4, port))
            },
            SOCKS5_ATYP_DOMAIN => {
                let mut len_buf = [0u8; 1];
                r.read_exact(&mut len_buf).await?;
                let domain_len = len_buf[0] as usize;
                let domain_and_port_len = (domain_len + 2) as usize;
                let mut buf = vec![0u8; domain_and_port_len];
                r.read_exact(&mut buf).await?;
                let domain_name = match String::from_utf8(Vec::from(&buf[..domain_len as usize])) {
                    Ok(x) => x,
                    Err(..) => return Err(Socks5Error::ParseError)
                };
                let port = unsafe {
                    u16::from_be(*(&buf[domain_len..] as *const _ as *const _))
                };
                Socks5Address::Domain(domain_name, port)
            },
            SOCKS5_ATYP_IPV6 => {
                let mut buf = [0u8, 18];
                r.read_exact(&mut buf).await?;
                let buf: &[u16] = unsafe { slice::from_raw_parts(&buf as *const _ as *const _, 9) };
                let ipv6 = Ipv6Addr::new(
                    u16::from_be(buf[0]),
                    u16::from_be(buf[1]),
                    u16::from_be(buf[2]),
                    u16::from_be(buf[3]),
                    u16::from_be(buf[4]),
                    u16::from_be(buf[5]),
                    u16::from_be(buf[6]),
                    u16::from_be(buf[7]),
                );
                let port = u16::from_be(buf[8]);
                Socks5Address::Ipv6Addr(SocketAddrV6::new(ipv6, port, 0, 0))
            }
            _ => return Err(Socks5Error::UnsupportedATYP(buf[0]))
        };
        Ok(addr)
    }
}

pub struct Socks5Request {
    command: Socks5Command,
    atyp: Socks5Address,
}

pub enum Socks5Command {
    CONNECT,
    BIND,
    UDP
}



// https://docs.rs/thiserror/1.0.26/thiserror/
#[derive(Debug, thiserror::Error)]
pub enum Socks5Error {
    // 用以将 io::Error 转换成 Socks5Error
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("unsupported socks version {0:#x}, expected 0x05")]
    UnsupportedSocks5Version(u8),
    #[error("unsupported authorization method {0:#x}, expected 0x00")]
    UnsupportedAuthorizationMethod(u8),
    #[error("unsupproted atype {0:#x}")]
    UnsupportedATYP(u8),
    #[error("unsupported command {0:#x}")]
    UnsupportedCommand(u8),
    #[error("parse occurred when decode bytes")]
    ParseError,
}