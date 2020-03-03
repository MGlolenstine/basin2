#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate basin2_lib;
#[macro_use]
extern crate scan_fmt;
pub use basin2_lib::result::*;

mod entity;
mod player;
mod server;
mod util;
mod world;

use tokio::runtime;
use log::LevelFilter;
use log::*;
use env_logger::Builder;
use std::sync::Arc;

fn start_tokio() -> Result<runtime::Runtime> {
    Ok(runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?)
}

pub fn start_server() {
    basin2_data::load(); // preload data now
    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
    let server = Arc::new(server::Server::new());
    start_tokio().unwrap().block_on(server::start(server))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_server() {
        super::start_server();
    }
}
