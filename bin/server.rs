use clap::clap_app;
use common::validate_type::validate_usize;
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
    let config = matches.value_of("CONFIG") {
        Some(path) => {

        }
    }
}
