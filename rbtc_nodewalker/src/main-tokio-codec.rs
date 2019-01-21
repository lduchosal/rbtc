extern crate mio;
extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

use futures::Stream;
use futures::Sink;
use futures::future::lazy;
use tokio::codec::*;
use tokio::io::{AsyncWrite, AsyncRead};
use tokio::net::{TcpStream, tcp::ConnectFuture};
use bytes::{Bytes, Buf, BytesMut};
use futures::{Future, Async, Poll};
use std::io::{self, Cursor};
use mio::Ready;

// TokioClient has two states, namely waiting to connect to the socket
// and already connected to the socket
struct TokioClient {
    state: State,

    retry_connect: u8,
    retry_loop: u8,

    connect: Option<ConnectFuture>, 
    framed: Option<Framed<TcpStream, LinesCodec>>, 

    buffer: Vec<String>,
}

enum State {
    Connecting,
    Connected,
    Receiving,
    Check,
    Respond,
    RetryConnect,
    Loop5,
    Reconnect,
}

enum HelloBranch {
    Continue,
    Notify(State),
    Stop
}

enum ReadState {
    Read,
    NeedMore,
    Failed
}

impl TokioClient {

    fn reconnect(&mut self) -> HelloBranch {
        println!("reconnect");
        self.connect = None;
        HelloBranch::Notify(State::Connecting)
    }

    fn connecting(&mut self) -> HelloBranch {
        
        println!("connecting");

        match self.connect {
            None => {
                let addr = "127.0.0.1:12345".parse().unwrap();
                let stream = TcpStream::connect(&addr);
                self.connect = Some(stream);
                self.buffer.truncate(0);
            },
            _ => {} ,
        };

        match self.connect {
            None => {
                HelloBranch::Notify(State::Connecting)
            },
            Some(ref mut socket) => {

                match socket.poll() {
                    Ok(Async::Ready(stream)) => {
                        println!("connecting [Ready]");

                        let framed = Framed::new(stream, LinesCodec::new());
                        self.framed = Some(framed);
                        self.retry_connect = 0;
                        //self.stream = Some(stream);
                        HelloBranch::Notify(State::Connected)

                    },
                    Ok(Async::NotReady) => {
                        println!("connecting [NotReady]");
                        HelloBranch::Continue
                    },
                    Err(e) => { 
                        println!("connecting [Err: {}]", e);
                        HelloBranch::Notify(State::RetryConnect)
                    }
                }
            }
        }
        
    }

    fn loop5(&mut self) -> HelloBranch {

        println!("loop5");
        println!("loop5 [retry: {}]", self.retry_loop);

        self.retry_loop = self.retry_loop + 1;
        let state = match self.retry_loop < 5 {
            true => State::Loop5,
            _ => State::Connecting
        };

        HelloBranch::Notify(state)
    }

    fn retry_connect(&mut self) -> HelloBranch {

        println!("retry_connect");
        println!("retry_connect [retry: {}]", self.retry_connect);

        std::thread::sleep_ms(500);
        let mut result = HelloBranch::Notify(State::Connecting);
        self.connect = None;
        self.retry_connect = self.retry_connect + 1;
        if self.retry_connect > 10 {
            result = HelloBranch::Stop;
        }
        result
    }

    fn connected(&mut self) -> HelloBranch {

        println!("connected");

        if let Some(ref mut framed) = self.framed {

            let mut send = framed.send("hello world".to_string());
            let state = match send.poll() {
                Ok(Async::Ready(_)) => {
                    println!("connected [Ready]");
                    HelloBranch::Notify(State::Receiving)
                },
                Ok(Async::NotReady) => {
                    println!("connected [NotReady]");
                    HelloBranch::Notify(State::Connected)
                },
                Err(e) => { 
                    println!("connected [Err: {}]", e);
                    HelloBranch::Notify(State::RetryConnect)
                }
            };
            return state;
        }
        HelloBranch::Notify(State::RetryConnect)
    }

    fn receiving(&mut self) -> HelloBranch {

        println!("receiving");

        if let Some(ref mut framed) = self.framed {

            let state = match framed.poll() {
                Ok(Async::Ready(Some(line))) => {
                    println!("receiving [Ready: {}]", line);
                    self.buffer.push(line);
                    HelloBranch::Notify(State::Check)
                },
                Ok(Async::Ready(None)) => {
                    println!("receiving [Ready: None]");
                    HelloBranch::Notify(State::RetryConnect)
                },
                Ok(Async::NotReady) => {
                    println!("receiving [NotReady]");
                    HelloBranch::Continue
                },
                Err(e) => { 
                    println!("receiving [Err: {}]", e);
                    HelloBranch::Notify(State::RetryConnect)
                }
            };
            return state;
        }
        HelloBranch::Notify(State::RetryConnect)

    }

    fn check(&mut self) -> HelloBranch {

        println!("Check");
        let mut result = HelloBranch::Notify(State::Respond);
        let expected = "quit".to_string();

        println!("Check [buffer: {}]", self.buffer.len());
        println!("Check [expected: {}]", expected);

        match self.buffer.pop() {

            None => {},
            Some(value) => {
                println!("Check [value: {}]", value);
                if *value == expected {
                    result = HelloBranch::Notify(State::Reconnect);
                }
            }
        }

        result
    }

    fn respond(&mut self) -> HelloBranch {

        println!("respond");

        if let Some(ref mut framed) = self.framed {

            let mut send = framed.send("sorry".to_string());
            let state = match send.poll() {
                Ok(Async::Ready(_)) => {
                    println!("respond [Ready]");
                    HelloBranch::Notify(State::Receiving)
                },
                Ok(Async::NotReady) => {
                    println!("respond [NotReady]");
                    HelloBranch::Notify(State::Connected)
                },
                Err(e) => { 
                    println!("respond [Err: {}]", e);
                    HelloBranch::Notify(State::RetryConnect)
                }
            };
            return state;
        }
        HelloBranch::Notify(State::Receiving)

    }
}

impl Future for TokioClient {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {

        let branch = match self.state {
            State::Connecting => self.connecting(),
            State::Loop5 => self.loop5(),
            State::RetryConnect => self.retry_connect(),
            State::Connected => self.connected(),
            State::Receiving => self.receiving(),
            State::Check => self.check(),
            State::Respond => self.respond(),
            State::Reconnect => self.reconnect(),
        };

        let current = futures::task::current();
        match branch {
            HelloBranch::Notify(state) => {

                self.state = state;
                let notifysoon = lazy(move || {
                    std::thread::sleep_ms(50);
                    current.notify();
                    Ok(())
                });

                tokio::spawn(notifysoon);
                Ok(Async::NotReady)
            },
            HelloBranch::Continue => {
                Ok(Async::NotReady)
            }
            HelloBranch::Stop => {
                Ok(Async::Ready(()))
            }
        }
    }
}

fn main() {

    let hello_world = TokioClient {
        state: State::Connecting,
        retry_connect: 0,
        retry_loop: 0,
        connect: None, 
        framed: None,
        buffer: Vec::new(),
    };

    // Run it, here we map the error since tokio::run expects a Future<Item=(), Error=()>
    tokio::run(hello_world.map_err(|e| println!("{0}", e)))
}