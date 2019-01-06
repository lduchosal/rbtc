extern crate chrono;

pub mod config;
pub mod resolver;
pub mod provider;
pub mod node;
pub mod walker;
pub mod program;

use std::path::Path;

fn main() {

    println!("rbtc_nodewalker 0.2.0 (q)");

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
    let walker = walker::NodeWalker::new();

    let program = program::Program::new(
        resolver,
        provider,
        walker
    );
    program.run();
}
