use crate::resolver;
use crate::provider;
use crate::walker;

use std::{thread, time};

pub struct Program {
    resolver: resolver::Resolver,
    provider: provider::NodeProvider,
    walker: walker::NodeWalker
}

impl Program {

    pub fn new(
        resolver: resolver::Resolver,
        provider: provider::NodeProvider,
        walker: walker::NodeWalker
    ) -> Program {
        Program {
            resolver: resolver,
            provider: provider,
            walker: walker,
        }
    }

    pub fn run(&self) {
        while true {
            self.seed();
            self.report();
            self.walk();

            let one_sec = time::Duration::from_secs(10);
            thread::sleep(one_sec);
        };
    }

    fn seed(&self) {

        let ips = self.resolver.ips()
            .into_iter()
            .map(|ip| ip.to_string())
            .collect()
            ;

        let src = String::from("dnsseed");
        self.provider.bulkinsert(ips, &src).unwrap();

    }

    fn report(&self) {

        let nodes = self.provider.all().unwrap();
        for node in nodes {
            println!("{} {} {}", node.id, node.src, node.ip);
        }

        let now = chrono::Utc::now();
        println!("------------------------------", );
        println!("{}", now.to_string());
        println!("------------------------------", );

    }

    fn walk(&self) {

        let nodes = self.provider.all().unwrap();
        for node in nodes {
            let src = node.ip;
            let ips = self.walker.walk(&src);
            self.provider.bulkinsert(ips, &src).unwrap();

        }
    }

}