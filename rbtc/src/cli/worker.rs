
use tokio::codec::length_delimited;
use tokio::codec::Framed;
use tokio::io::{AsyncWrite, AsyncRead};
use tokio::net::{TcpStream, tcp::ConnectFuture};
use tokio::prelude::task::Spawn;

use futures::{Future, Async, Poll, Sink, Stream};
use futures::future::lazy;
use futures::future::Executor;
use futures::sync::mpsc::{channel, Receiver, Sender};

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
        SetAddr => SettingAddr
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
    result: std::sync::mpsc::Sender<Result<SocketAddr, Error>>,
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

        println!("change_state");
        println!("change_state [request: {:#?}]", request);
        println!("change_state [state: {:#?}]", self.state);

        let next = match request {
            Request::SetAddr(_) => self.state.set_addr(),
            Request::Connect(_) => self.state.connect(),
        };

        println!("change_state [next: {:#?}]", next);
        println!("change_state [state: {:#?}]", self.state);

        match next {
            None => {},
            Some(State::Init) => {},
            Some(State::SettingAddr) => {
                match request {
                    Request::SetAddr(setaddr) => {
                        let setaddr = match self.set_addr(setaddr) {
                            Ok(_) => self.state.succeed(),
                            Err(_) => self.state.failed(),
                        };
                        println!("change_state [setaddr: {:#?}]", setaddr);
                    },
                    _ => {}
                }
            }
            Some(State::SetAddr) => {},
            Some(State::Connecting) => {},
            Some(State::Connected) => {},
            Some(State::__InvalidState__) => {}
        };

        println!("change_state [state: {:#?}]", self.state);

    }

    fn set_addr(&mut self, request: SetAddrRequest) -> Result<(), ()>{

        println!("Worker set_addr [request: {:#?}]", request);

        let (sender, response) = std::sync::mpsc::channel::<Result<SocketAddr, Error>>();
        let setaddr = SetAddrWorker {
            addr: request.addr,
            result: sender,
        };
<<<<<<< HEAD
=======

>>>>>>> 562b12bc42a390dce77eba60dff884c14e90eaa4
        let spawn = tokio::spawn(setaddr);
        println!("Worker set_addr [spawn: {:#?}]", spawn);
        
        let result = match response.recv() {
            Ok(Ok(addr)) => {
                self.addr = Some(addr);
                Ok(())
            },
            Ok(Err(err)) => {
                let error = format!("{:#?}", err);
                Err(Error::SetAddrResponseFailed(error))
            },
            Err(err) => {
                Err(Error::SetAddrResponseFailed(err.to_string()))
            }
        };

<<<<<<< HEAD
        drop(response);

        println!("set_addr [result: {:#?}]", result);
=======
        println!("Worker set_addr [result: {:#?}]", result);
>>>>>>> 562b12bc42a390dce77eba60dff884c14e90eaa4
        let sent = request.sender.send(result);
        println!("Worker set_addr [sent: {:#?}]", sent);

        Ok(())
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
        println!("connect [response: {:#?}]", response);

        let sent = request.sender.send(response);
        println!("connect [sent: {:#?}]", sent);
        Ok(Async::Ready(()))
    }

}

impl Future for SetAddrWorker {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        println!("SetAddrWorker poll");
        println!("SetAddrWorker poll [addr: {}]", self.addr);

        let mut node_ip_port = self.addr.to_string();

        let result = match node_ip_port.parse() {
            Ok(addr) => {
                println!("SetAddrWorker poll [ok1]");
                Ok(addr)
            },
            Err(_) => {
                
                node_ip_port.push_str(":8333");
                match node_ip_port.parse() {
                    Ok(addr) => {
                        println!("SetAddrWorker poll [ok2]");
                        Ok(addr)
                    },
                    Err(err) => {
                        println!("SetAddrWorker poll [err: {}]", err);
                        println!("SetAddrWorker poll [node_ip_port: {}]", node_ip_port);
                        let sockerr: AddrParseError = err;
                        Err(Error::SetAddrResponseFailed(sockerr.to_string()))
                    }
                }
            }
        };
        let sent = self.result.send(result);
        println!("SetAddrWorker poll [sent: {:#?}]", sent);

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
                    //futures::task::current().notify();
                    //return Ok(Async::NotReady);
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
        println!("Worker end");

        Ok(Async::Ready(()))
    }
}