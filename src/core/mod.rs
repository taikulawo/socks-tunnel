use std::{io, sync::Arc};

use trust_dns_resolver::{Resolver, config::{ResolverConfig, ResolverOpts}};

use crate::{core::config::ServerConfig, service::config::Config};

pub mod config;
pub mod copy;
mod tcp_relay;
mod socks5;


pub struct Context {
    pub resolver: Resolver
}

pub struct MainInstance {
    pub context: Arc<Context>
}

impl MainInstance {
    pub fn new() -> MainInstance{
        let resolver = Resolver::new(ResolverConfig::default(),ResolverOpts::default()).unwrap();
        MainInstance {
            context: Arc::new(Context {
                resolver
            })
        }
    }
}

pub async fn start_to_run_server(config: Config) -> io::Result<()>{
    Ok(())
}