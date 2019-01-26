
use std::sync::mpsc::SendError;
use std::sync::mpsc::RecvError;
use crate::cli::*;
use crate::cli::worker::Worker;
use crate::cli::result::*;

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

use std::time;
use std::thread;
use futures::sync::mpsc::{channel, Receiver, Sender};
use futures::{Sink, Future};

#[derive(Debug)]
pub enum Request {
    SetAddr(String)
}

#[derive(Debug)]
pub enum Response {
    SetAddr(bool)
}

pub struct Rbtc {
    send: Sender<Request>,
    recv: Receiver<Response>,
}

impl Rbtc {
    
    pub fn new() -> Rbtc {

        trace!("new");

        let (send_request, rcv_request) = channel::<Request>(1);
        let (send_response, rcv_response) = channel::<Response>(1);

        std::thread::spawn(move || {
            println!("pool.spawn");
            let worker = Worker::new(rcv_request, send_response);
            tokio::run(worker);
        });

        Rbtc {
            send: send_request,
            recv: rcv_response
        }
    }

    pub fn set_addr(&mut self, addr: String) -> Result<(), ()> {

        println!("set_addr");
        let request = Request::SetAddr(addr);
        self.send.clone()
            .send(request)
            .wait()
            .map(|_sender| ())
            .map_err(|_err| ())
            
    }
}
