use crate::resolver;
use crate::provider;
use crate::walker;
use crate::fsm::WalkerSmEvents;

use std::{thread, time};

pub struct Program {
    resolver: resolver::Resolver,
    provider: provider::NodeProvider,
}

impl Program {

    pub fn new(
        resolver: resolver::Resolver,
        provider: provider::NodeProvider,
    ) -> Program {
        Program {
            resolver: resolver,
            provider: provider,
        }
    }

    pub fn run(&self) {

        loop {

            self.report();
            self.seed();
            self.report();
            self.walk();
            self.report();

            let ten_sec = time::Duration::from_secs(10);
            thread::sleep(ten_sec);
        }
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
        // for node in nodes {
        //     println!("{} {} {}", node.id, node.src, node.ip);
        // }
        println!("Node capture : {}", nodes.len());

        let now = chrono::Local::now();
        println!("------------------------------", );
        println!("{}", now.to_string());
        println!("------------------------------", );

    }

    fn walk(&self) {

        let nodes = self.provider.all().unwrap();
        for node in nodes {

            let src = node.ip;
            let mut walker = walker::NodeWalker::new(&src);
            walker.run();
            
            let ips = walker.ips();
            println!("NodeWalker got {} new ips from {}", ips.len(), src);
            let inserted = self.provider.bulkinsert(ips, &src);

            if let Err(err) = inserted {
                println!("NodeProvider failed with : {}", err);
                continue;
            }

        }
    }

}