
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
use futures::sync::mpsc;
use futures::sync::oneshot;
use futures::{Sink, Future, Stream};

pub struct Rbtc {
    send: mpsc::Sender<Request>,
}

pub enum Request {
    SetAddr(SetAddrRequest)
}

pub struct SetAddrRequest {
    pub addr: String,
    pub sender: oneshot::Sender<SetAddrResponse>
}

pub struct SetAddrResponse {
    pub result: bool,
}

impl Rbtc {
    
    pub fn new() -> Rbtc {

        trace!("new");

        let (send_request, rcv_request) = mpsc::channel::<Request>(1);

        std::thread::spawn(move || {
            println!("pool.spawn");
            let worker = Worker::new(rcv_request);
            tokio::run(worker);
        });

        Rbtc {
            send: send_request,
        }
    }

    pub fn set_addr(&mut self, addr: String) -> impl Future<Item=bool, Error=oneshot::Canceled> {
        let (sender, response) = oneshot::channel::<SetAddrResponse>();

        println!("set_addr");
        let setaddr = SetAddrRequest {
            addr: addr,
            sender: sender
        };
        let request = Request::SetAddr(setaddr);
        self.send.clone()
            .send(request)
            .wait()
            ;
        
        response.map(|res| res.result)
    }

}
