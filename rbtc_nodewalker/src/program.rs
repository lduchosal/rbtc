use crate::resolver;
use crate::provider;
use crate::walker::*;
use crate::walker::result::*;
use crate::walker::walker::NodeWalker;
use crate::walker::walker::WalkResult;
use crate::walker::fsm::WalkerFsmEvents;
use crate::node;

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
    sender: Sender<WalkResult>
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

    pub fn run(&mut self) {

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

    fn seed(&mut self) {

        trace!("seed");

        let ips = self.resolver.ips()
            .into_iter()
            .map(|ip| ip.to_string())
            .collect()
            ;

        let src = String::from("dnsseed");
        self.provider.bulkinsert(ips, &src, 0).unwrap();

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

    fn walk(&mut self) {

        trace!("walk");

        let (sender, receiver) : (Sender<WalkResult>, Receiver<WalkResult>)= channel();

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

                let id = node.id;
                let src = node.ip.clone();
                info!("walk [id: {}]", id);
                info!("walk [src: {}]", src);

                let mut walker = walker::NodeWalker::new(id, &src);
                walker.run();

                let result = walker.result();
                sender.send(result);

                drop(sender);
            });

        drop(sender);

        while let Ok(walkresult) = receiver.recv() {
            trace!("walk [rcv]");
            self.end(walkresult);
        }
    }

    fn end(&mut self, walkresult: WalkResult) {
    
        trace!("end");
        debug!("end [src: {:?}]", walkresult.src);
        debug!("end [result: {:?}]", walkresult.result);
        debug!("end [ips: {:?}]", walkresult.ips.len());

        match &walkresult.result {
            None => error!("end [result: None]"),
            Some(result) => {
                match result {
                    EndResult::ParseAddr => self.insert(walkresult),
                    EndResult::ParseAddrFailed => self.delete(walkresult),
                    EndResult::RetryFailed =>  self.delete(walkresult),
                    EndResult::SendVersionFailed => self.deactivate(walkresult),
                    EndResult::ReceiveVersionFailed => self.deactivate(walkresult),
                    EndResult::ReceiveVerackFailed => self.deactivate(walkresult),
                    EndResult::SendVerackFailed => self.deactivate(walkresult),
                    EndResult::SendGetAddrRetryFailed => self.deactivate(walkresult),
                }
            }
        }
    }

    fn insert(&mut self, walkresult: WalkResult) {
        trace!("insert");

        let src = &walkresult.src;
        let ips = walkresult.ips;
        let id = walkresult.id;

        debug!("delete [id: {}]", id);
        debug!("delete [src: {}]", src);
        debug!("insert [ips: {}]", ips.len());

        let inserted = self.provider.bulkinsert(ips, src, id);
        if let Err(err) = inserted {
            error!("insert [err: {}]", err);
        }
    }

    fn delete(&mut self, walkresult: WalkResult) {
        trace!("delete");
        let id = walkresult.id;
        debug!("delete [src: {}]", id);

        let deleted = self.provider.delete(id);
        if let Err(err) = deleted {
            error!("delete [err: {}]", err);
        }

    }

    fn deactivate(&mut self, walkresult: WalkResult) {
        trace!("deactivate");
        let id = walkresult.id;
        debug!("deactivate [id: {}]", id);

        let deactivate = self.provider.deactivate(id);
        if let Err(err) = deactivate {
            error!("deactivate [err: {}]", err);
        }

    }
}