
use tokio::codec::length_delimited;
use tokio::codec::Framed;
use tokio::io::{AsyncWrite, AsyncRead};
use tokio::net::{TcpStream, tcp::ConnectFuture};

use futures::{Future, Async, Poll, Sink, Stream};
use futures::future::lazy;
use futures::sync::mpsc::{Receiver, Sender};
use futures::sync::oneshot;

use crate::cli::*;
use crate::cli::result::*;
use crate::cli::rbtc::*;

use std::net::{SocketAddr, AddrParseError};
use std::io::prelude::*;

use std::time;

use self::RbtcFsm::State;

microstate!{

    RbtcFsm { Init }

    states { Init, SettingAddr, SetAddr, Connecting, Connected }

    set_addr {
        Init => SettingAddr
    }

    connect {
        SetAddr => Connecting
    }

    succeed {
        SettingAddr => SetAddr
        Connecting => Connected
    }

    failed {
        SettingAddr => Init
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

struct SetAddrWorker {
    addr: String,
    result: oneshot::Sender<Result<SocketAddr, Error>>,
}

struct Connect {
    worker: Worker,
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

    fn change_state(&mut self, request: Request) {

        let next = match request {
            Request::SetAddr(_) => self.state.set_addr(),
            Request::Connect(_) => self.state.connect(),
        };

        match next {
            None => {},
            Some(State::Init) => {},
            Some(State::SettingAddr) => {
                match request {
                    Request::SetAddr(setaddr) => {
                        match self.set_addr(setaddr) {
                            Ok(_) => self.state.succeed(),
                            Err(_) => self.state.failed(),
                        };
                    },
                    _ => {}
                }
            }
            Some(State::SetAddr) => {},
            Some(State::Connecting) => {},
            Some(State::Connected) => {},
            Some(State::__InvalidState__) => {}
        };

    }

    fn set_addr(&mut self, request: SetAddrRequest) -> Result<(), ()>{

        let (sender, response) = oneshot::channel::<Result<SocketAddr, Error>>();
        let setaddr = SetAddrWorker {
            addr: request.addr,
            result: sender,
        };
        tokio::spawn(setaddr);
        
        match response.wait() {
            Ok(Ok(addr)) => {
                self.addr = Some(addr);
                request.sender.send(Ok(()));
                Ok(())
            },
            Ok(Err(err)) => {
                request.sender.send(Err(Error::SetAddrResponseFailed("set_addr err".to_string())));
                Err(())
            },
            Err(err) => {
                request.sender.send(Err(Error::SetAddrResponseFailed(err.to_string())));
                Err(())
            }
        }
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

impl Future for SetAddrWorker {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        trace!("poll");
        debug!("poll [addr: {}]", self.addr);

        let mut node_ip_port = self.addr.to_string();

        let result = match node_ip_port.parse() {
            Ok(addr) => {
                Ok(addr)
            },
            Err(_) => {
                
                node_ip_port.push_str(":8333");
                match node_ip_port.parse() {
                    Ok(addr) => {
                        trace!("set_addr [ok]");
                        Ok(addr)
                    },
                    Err(err) => {
                        warn!("set_addr [err: {}]", err);
                        warn!("set_addr [node_ip_port: {}]", node_ip_port);
                        let sockerr: AddrParseError = err;
                        Err(Error::SetAddrResponseFailed(sockerr.to_string()))
                    }
                }
            }
        };
        self.result.send(result);
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
                    self.change_state(request);
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