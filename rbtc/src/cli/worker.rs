
use futures::{Future, Async, Poll, Sink, Stream};
use futures::future::lazy;
use futures::sync::mpsc::{Receiver, Sender};

use tokio::io::{AsyncWrite, AsyncRead};

use crate::cli::*;
use crate::cli::result::*;
use crate::cli::rbtc::*;

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;

use std::time;

microstate!{

    RbtcFsm { Init }

    states { Init, SetAddr }

    set_addr {
        Init => SetAddr
    }
}

pub struct Worker {

    state: RbtcFsm::Machine,
    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,

    recv: Receiver<Request>,
    send: Sender<Response>,

}

impl Worker {

    fn state(&self) -> RbtcFsm::State {
        self.state.state()
    }

    pub(crate) fn new(recv: Receiver<Request>, send: Sender<Response>) -> Worker {

        println!("new");

        let node_ip_port = "127.0.0.1:8333".to_string();
        let state = RbtcFsm::new();

        Worker {
            state: state,
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            stream: None,
            
            recv: recv,
            send: send
        }
    }

    pub fn set_addr(&mut self, addr: &str) {

        trace!("do_set_addr");
        debug!("do_set_addr [addr: {}]", addr);

        let mut node_ip_port = addr.to_string();
        let mut result = SetAddrResult::ParseAddrFailed;

        if let Ok(addr) = node_ip_port.parse() {
            self.addr = Some(addr);
            result = SetAddrResult::Succeed;
        } 
        else {
            node_ip_port.push_str(":8333");
            result = match node_ip_port.parse() {
                Ok(addr) => {
                    trace!("do_set_addr [ok]");
                    self.addr = Some(addr);
                    SetAddrResult::Succeed
                },
                Err(err) => {
                    warn!("do_set_addr [err: {}]", err);
                    warn!("do_set_addr [node_ip_port: {}]", node_ip_port);
                    SetAddrResult::ParseAddrFailed
                }
            }
        }
    }
}


impl Future for Worker {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        // let action = match self.state() {
        //     RbtcFsm::State::Init => { Action::Notify },
        //     RbtcFsm::State::SetAddr => { Action::Notify },
        // };

        println!("poll");

        loop {
            match self.recv.poll() {
                Ok(Async::Ready(Some(request))) => {
                    println!("Async::Ready(Some) {:#?}", request);
                    let next = match request {
                        Request::SetAddr(ref addr) => {
                            self.set_addr(addr)
                        }
                    };
                },
                Ok(Async::Ready(None)) => {
                    println!("Async::Ready(None)");
                },
                Ok(Async::NotReady) => {
                    println!("Async::NotReady");
                    return Ok(Async::NotReady);
                },
                Err(err) => {
                    println!("Err(err), {:#?}", err);
                    break;
                }
            }
        }

        Ok(Async::Ready(()))
    }
}