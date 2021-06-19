use std::net::SocketAddr;


#[derive(Debug)]
pub enum ServerAddr {
    SocketAddr(SocketAddr),
    DomainName(String, u16)
}


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
