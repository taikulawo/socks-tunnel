use std::fmt::format;

use clap::clap_app;
use common::validate_type::validate_usize;
use futures::{future, stream::FuturesUnordered};
use socks_tunnel::{core::start_to_run_server, service::config::Config};
mod common;

const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let mut app = clap_app!( socksshadow => 
        (version: VERSION)
        (@arg CONFIG: -c --config +takes_value required_unless("SERVER_ADDR") "Shadowsocks configuration file")
        (@arg SERVER_ADDR: -s --("server-addr") +takes_value "Bind address")
    );
    app = clap_app!(@app (app)
        (@arg WORKER_THREADS: --("worker-threads") +takes_value {validate_usize} "sets the number of worker threads the `Runtime` will use")
    );
    let matches = app.get_matches();
    let config = match matches.value_of("CONFIG") {
        Some(path) => {
            match Config::load_config_from_file("") {
                Ok(c) => c,
                Err(e) => panic!(e)
            }
        },
        None => return ()
    };
    let rt = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(err) => panic!(err)
    };
    rt.block_on(async move {
        let vfuts = FuturesUnordered::new();
        let vfut = start_to_run_server(config);
        let vfut = Box::pin(vfut);
        vfuts.push(vfut);
        match future::select_all(vfuts).await {
            (Ok(..), idx, ..) => {},
            (Err(err), idx, ..) => {
                panic!("future run exit at {}", err)
            }
        }
    });
}
