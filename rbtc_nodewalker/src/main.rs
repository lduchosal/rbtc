extern crate chrono;
extern crate rayon;
extern crate rand;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod config;
pub mod resolver;
pub mod provider;
pub mod node;
pub mod fsm;
pub mod walker;
pub mod program;
pub mod message;

use std::path::Path;

fn main() {

    pretty_env_logger::init();

    info!("rbtc_nodewalker 0.3.0 (q)");

    let config = config::Config {
        dns_seeds: vec![
            String::from("seed.bitcoin.sipa.be"),
            String::from("dnsseed.bluematt.me"),
            String::from("dnsseed.bitcoin.dashjr.org"),
            String::from("seed.bitcoinstats.com"),
            String::from("seed.bitcoin.jonasschnelli.ch"),
            String::from("seed.btc.petertodd.org"),
            String::from("seed.bitcoin.sprovoost.nl"),
        ],
        sqlite_path: Path::new("./nodes.sqlite"),
    };

    let resolver = resolver::Resolver::new(config.dns_seeds);
    let provider = provider::NodeProvider::new(&config.sqlite_path).unwrap();

    let mut program = program::Program::new(
        resolver,
        provider
    );
    program.run();
}
