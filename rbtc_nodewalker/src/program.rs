use crate::resolver;
use crate::provider;
use crate::walker;
use crate::node;
use crate::fsm::WalkerSmEvents;

use rayon::prelude::*;

use std::{thread, time};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


pub struct Program {
    resolver: resolver::Resolver,
    provider: provider::NodeProvider,
}

pub struct Comm {
    node: node::Node,
    sender: Sender<(String, Vec<String>)>
}

impl Program {

    pub fn new(
        resolver: resolver::Resolver,
        provider: provider::NodeProvider,
    ) -> Program {

        trace!("new");

        Program {
            resolver: resolver,
            provider: provider,
        }
    }

    pub fn run(&self) {

        trace!("run");

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

        trace!("seed");

        let ips = self.resolver.ips()
            .into_iter()
            .map(|ip| ip.to_string())
            .collect()
            ;

        let src = String::from("dnsseed");
        self.provider.bulkinsert(ips, &src).unwrap();

    }

    fn report(&self) {

        trace!("report");

        let nodes = self.provider.all().unwrap();
        debug!("Node capture : {}", nodes.len());

        let now = chrono::Local::now();
        info!("------------------------------", );
        info!("{}", now.to_string());
        info!("------------------------------", );

    }

    fn walk(&self) {

        trace!("walk");

        let (sender, receiver) : (Sender<(String, Vec<String>)>, Receiver<(String, Vec<String>)>)= channel();

        let nodes = self.provider.ten()
            .unwrap();

        let comms : Vec<Comm> = nodes
            .into_iter()
            .map(|node| Comm {
                node: node,
                sender: sender.clone()
            })
            .collect();

        comms.into_par_iter()
            .for_each( |comm| {

                let node = comm.node;
                let sender = comm.sender;

                let src = node.ip.clone();
                info!("walk [src: {}]", src);
                let mut walker = walker::NodeWalker::new(&src);
                walker.run();
                
                let result = walker.ips();
                info!("walk [result: {}]", result.len());
                sender.send((src, result));

            });

        while let Ok((src, ips)) = receiver.recv() {
                warn!("walk [rcv]");

            let inserted = self.provider.bulkinsert(ips, &src);
            if let Err(err) = inserted {
                warn!("walk [err: {}]", err);
            }
        }

    }

}