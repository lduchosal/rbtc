pub mod config;
pub mod resolver;
pub mod node;

use std::path::Path;

fn main() {

    println!("rbtc_nodewalker 0.2.0 (q)");

    let config = config::Config {
        dns_seeds: vec![
            "seed.bitcoin.sipa.be",
            "dnsseed.bluematt.me",
            "dnsseed.bitcoin.dashjr.org",
            "seed.bitcoinstats.com",
            "seed.bitcoin.jonasschnelli.ch",
            "seed.btc.petertodd.org",
            "seed.bitcoin.sprovoost.nl",
        ],
        sqlite_path: Path::new("./nodes.sqlite"),
    };

    let resolver = resolver::Resolver::new(&config);
    let provider = node::NodeProvider::new(&config.sqlite_path).unwrap();

    
    let ips = resolver.ips()
        .into_iter()
        .map(|ip| ip.to_string())
        .collect()
        ;

    let src = String::from("dnsseed");

    provider.bulkinsert(ips, src);

    let nodes = provider.all().unwrap();
    
    for node in nodes {
        println!("{} {} {}", node.id, node.ip, node.src);
    }

}
