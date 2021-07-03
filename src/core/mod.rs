use std::io;

use crate::{core::config::ServerConfig, service::config::Config};

pub mod config;
pub mod copy;
mod tcp_relay;
mod socks5;

pub async fn start_to_run_server(config: Config) -> io::Result<()>{
    Ok(())
}