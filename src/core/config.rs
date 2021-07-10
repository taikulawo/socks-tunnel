use std::net::SocketAddr;


#[derive(Debug, Clone)]
pub enum ServerAddr {
    SocketAddr(SocketAddr),
    DomainName(String, u16)
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    server_address: ServerAddr,
    password: String,
}

impl ServerConfig {
    pub fn new(host: ServerAddr, password: String) -> ServerConfig {
        return ServerConfig {
            password,
            server_address: host,
        }
    }
}
