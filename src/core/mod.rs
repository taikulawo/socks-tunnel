use std::{io, sync::Arc};

use crate::{
    core::{config::ServerConfig, tcp_relay::Server},
    service::config::Config,
};
use futures::{future, stream::FuturesUnordered, FutureExt};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub mod config;
pub mod copy;
mod socks5;
mod tcp_relay;
mod nat;
pub use nat::NetworkTranslator;
pub struct Context {
    pub resolver: Resolver,
}

pub async fn start_to_run_server(config: Config) -> io::Result<()> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let future_container = FuturesUnordered::new();
    let ctx = Arc::new(Context { resolver });
    for c in &config.server_addr {
        let server = Server::new(c.clone(), ctx.clone());
        future_container.push(server.run_server().boxed());
    }
    let (res, ..) = future::select_all(future_container).await;
    res
}
