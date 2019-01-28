
use tokio::codec::length_delimited;
use tokio::codec::Framed;
use tokio::io::{AsyncWrite, AsyncRead};
use tokio::net::{TcpStream, tcp::ConnectFuture};

use futures::{Future, Async, Poll, Sink, Stream};
use futures::future::lazy;
use futures::sync::mpsc::{Receiver, Sender};

use crate::cli::*;
use crate::cli::result::*;
use crate::cli::rbtc::*;

use std::net::{SocketAddr};
use std::io::prelude::*;

use std::time;

microstate!{

    RbtcFsm { Init }

    states { Init, SetAddr, Connecting, Connected }

    set_addr {
        Init => SetAddr
    }

    connect {
        SetAddr => Connecting
    }

    connect_succeed {
        Connecting => Connected
    }

    connect_failed {
        Connecting => SetAddr
    }
}

pub struct Worker {

    state: RbtcFsm::Machine,
    recv: Receiver<Request>,

    connect_retry: u8,
    getaddr_retry: u8,

    node_ip_port: String,
    addr: Option<SocketAddr>,

    connect: Option<ConnectFuture>, 
    framed: Option<Framed<TcpStream, length_delimited::LengthDelimitedCodec>>, 

}

impl Worker {

    pub(crate) fn new(recv: Receiver<Request>) -> Worker {

        println!("new");

        let node_ip_port = "127.0.0.1:8333".to_string();
        let state = RbtcFsm::new();

        Worker {
            state: state,
            recv: recv,

            connect_retry: 0,
            getaddr_retry: 0,

            node_ip_port: node_ip_port,
            addr: None,

            connect: None,
            framed: None,

        }
    }

    fn change_state(&mut self, request: Request) -> Poll<(), ()>{

        self.state.
        let next = match request {
            Request::SetAddr(setaddr) => self.set_addr(setaddr),
            Request::Connect(connect) => self.connect(connect),
        };
    }

    fn set_addr(&mut self, request: SetAddrRequest) -> Poll<(), ()> {

        trace!("set_addr");
        debug!("set_addr [addr: {}]", request.addr);

        let mut node_ip_port = request.addr.to_string();

        let response = match node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                Ok(())
            },
            Err(_) => {
                
                node_ip_port.push_str(":8333");
                match node_ip_port.parse() {
                    Ok(addr) => {
                        trace!("set_addr [ok]");
                        self.addr = Some(addr);
                        Ok(())
                    },
                    Err(err) => {
                        warn!("set_addr [err: {}]", err);
                        warn!("set_addr [node_ip_port: {}]", node_ip_port);
                        Err(Error::SetAddrResponseFailed(err.to_string()))
                    }
                }
            }
        }; 
        let sent = request.sender.send(response);
        Ok(Async::Ready(()))
    }

    fn connect(&mut self, request: ConnectRequest) -> Poll<(), ()> {
        
        println!("connect");

        match self.connect {
            None => {
                let stream = TcpStream::connect(&self.addr.unwrap());
                self.connect = Some(stream);
            },
            _ => {},
        };

        let response = match self.connect {
            None => {
                Ok(())
            },
            Some(ref mut socket) => {

                match socket.poll() {
                    Ok(Async::Ready(stream)) => {
                        println!("connecting [Ready]");

                        let ld = length_delimited::Builder::new()
                            .length_field_offset(16) // bitcoin header + message
                            .length_field_length(2)
                            .length_adjustment(-16)   // default value
                            .num_skip(2) // Do not strip frame header
                        ;
                        // let framed = Framed::new(stream, ld);
                        //self.framed = Some(framed);
                        Ok(())
                    },
                    Ok(Async::NotReady) => {
                        println!("connecting [NotReady]");
                        return Ok(Async::NotReady);
                    },
                    Err(e) => { 
                        println!("connecting [Err: {}]", e);
                        Err(Error::CommectFailed(e.to_string()))
                    }
                }
            }
        };

        let sent = request.sender.send(response);
        Ok(Async::Ready(()))
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

        println!("Worker poll");

        loop {
            match self.recv.poll() {
                Ok(Async::Ready(Some(request))) => {
                    println!("Worker Async::Ready(Some)");
                    let next = self.change_state(request);
                    println!("Worker Async::Ready(Some) [{:#?}]", next);
                    match next {
                        Ok(Async::Ready(_)) => {},
                        Ok(Async::NotReady) => {
                            return Ok(Async::NotReady);
                        },
                        Err(_) => {},
                    }
                },
                Ok(Async::Ready(None)) => {
                    println!("Worker Async::Ready(None)");
                },
                Ok(Async::NotReady) => {
                    println!("Worker Async::NotReady");
                    return Ok(Async::NotReady);
                },
                Err(err) => {
                    println!("Worker Err(err), {:#?}", err);
                    break;
                }
            }
        }

        Ok(Async::Ready(()))
    }
}