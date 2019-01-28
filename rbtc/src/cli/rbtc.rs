
use std::net::AddrParseError;
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
use futures::future::lazy;
use futures::sync::mpsc;
use futures::sync::oneshot;
use futures::{Sink, Future, Stream};

pub struct Rbtc {
    send: mpsc::Sender<Request>,
}

pub enum Request {
    SetAddr(SetAddrRequest),
    Connect(ConnectRequest)
}

#[derive(Debug)]
pub struct SetAddrRequest {
    pub addr: String,
    pub sender: oneshot::Sender<Result<(), Error>>
}

#[derive(Debug)]
pub struct ConnectRequest {
    pub sender: oneshot::Sender<Result<(), Error>>
}

#[derive(Debug)]
pub enum Error {
    None,
    SetAddrResponseFailed(String),
    CommectFailed(String),
}

impl Rbtc {
    
    pub fn new() -> Rbtc {

        trace!("new");

        let (send_request, rcv_request) = mpsc::channel::<Request>(1);

        std::thread::spawn(move || {
            println!("worker.spawn");
            let worker = Worker::new(rcv_request);
            tokio::run(worker);
        });

        Rbtc {
            send: send_request,
        }
    }

    pub fn set_addr(&mut self, addr: String) -> impl Future<Item=Result<(), Error>, Error=oneshot::Canceled> {
        let (sender, response) = oneshot::channel::<Result<(), Error>>();

        println!("set_addr");
        let setaddr = SetAddrRequest {
            addr: addr,
            sender: sender
        };
        let request = Request::SetAddr(setaddr);
        let sent = self.send.clone()
            .send(request)
            .wait()
            ;
        
        match sent {
            Ok(_) => {},
            Err(err) => println!("set_addr [err: {}]", err)
        };

        response
    }


    pub fn connect(&mut self) -> impl Future<Item=Result<(), Error>, Error=oneshot::Canceled> {
        let (sender, response) = oneshot::channel::<Result<(), Error>>();

        println!("connect");
        let connect = ConnectRequest {
            sender: sender
        };
        let request = Request::Connect(connect);
        let sent = self.send.clone()
            .send(request)
            .wait()
            ;
        
        match sent {
            Ok(_) => {},
            Err(err) => println!("connect [err: {}]", err)
        };

        response
    }

}
