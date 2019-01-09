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
        self.seed();
        self.report();
        self.walk();

        let ten_sec = time::Duration::from_secs(10);
        thread::sleep(ten_sec);
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

        let now = chrono::Local::now();
        println!("------------------------------", );
        println!("{}", now.to_string());
        println!("------------------------------", );

    }

    fn walk(&self) {

        let nodes = self.provider.all().unwrap();
        for node in nodes {
            let src = node.ip;
            let walked = self.walker.walk(&src);
            if let Err(err) = walked {
                println!("NodeWalker failed with : {}", err);
                continue;
            }

            let ips = walked.unwrap();
            let inserted = self.provider.bulkinsert(ips, &src);

            if let Err(err) = inserted {
                println!("NodeProvider failed with : {}", err);
                continue;
            }

        }
    }

}