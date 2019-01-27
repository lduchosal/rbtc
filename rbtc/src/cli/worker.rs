
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

}

impl Worker {

    fn state(&self) -> RbtcFsm::State {
        self.state.state()
    }

    pub(crate) fn new(recv: Receiver<Request>) -> Worker {

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
        }
    }

    pub fn set_addr(&mut self, request: SetAddrRequest) {

        trace!("set_addr");
        debug!("set_addr [addr: {}]", request.addr);

        let mut node_ip_port = request.addr.to_string();

        let response = match node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                SetAddrResponse { result: true }
            },
            Err(_) => {
                
                node_ip_port.push_str(":8333");
                match node_ip_port.parse() {
                    Ok(addr) => {
                        trace!("set_addr [ok]");
                        self.addr = Some(addr);
                        SetAddrResponse { result: true }
                    },
                    Err(err) => {
                        warn!("set_addr [err: {}]", err);
                        warn!("set_addr [node_ip_port: {}]", node_ip_port);
                        SetAddrResponse { result: false }
                    }
                }
            }
        }; 
        request.sender.send(response);

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
                    println!("Async::Ready(Some)");
                    let next = match request {
                        Request::SetAddr(setaddr) => self.set_addr(setaddr)
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