use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{self, Read},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::Path,
    vec,
};

use crate::core::config::{ServerAddr, ServerConfig};

pub struct Config {
    pub server_addr: Vec<ServerConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SSConfigFromJSON {
    server_host: Option<String>,
    server_port: Option<u16>,
}

impl Config {
    pub fn load_config_from_file(path: &str) -> io::Result<Config> {
        let mut reader = OpenOptions::new().read(true).open(Path::new(path))?;
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Config::load_from_str(&*content)
    }
    pub fn load_from_str(content: &str) -> io::Result<Config> {
        let config: SSConfigFromJSON = serde_json::from_str(content)?;
        Config::load_from_server_config(config)
    }

    pub fn new() -> Config {
        Config {
            server_addr: Vec::new(),
        }
    }
    fn load_from_server_config(config: SSConfigFromJSON) -> io::Result<Config> {
        let addr = match (config.server_host, config.server_port) {
            (Some(host), Some(port)) => match host.parse::<Ipv4Addr>() {
                Ok(v4) => ServerAddr::SocketAddr(SocketAddr::V4(SocketAddrV4::new(v4, port))),
                Err(..) => match host.parse::<Ipv6Addr>() {
                    Ok(v6) => {
                        ServerAddr::SocketAddr(SocketAddr::V6(SocketAddrV6::new(v6, port, 0, 0)))
                    }
                    Err(..) => ServerAddr::DomainName(host, port),
                },
            },
            _ => return Err(io::Error::new(io::ErrorKind::Other, "address parse failed")),
        };
        let server_configs = ServerConfig::new(addr, String::from(""));
        Ok(Config {
            server_addr: vec![server_configs],
        })
    }
}
