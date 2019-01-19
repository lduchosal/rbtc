extern crate mio;
extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

use futures::future::lazy;
use tokio::io::{AsyncWrite, AsyncRead};
use tokio::net::{TcpStream, tcp::ConnectFuture};
use bytes::{Bytes, Buf, BytesMut};
use futures::{Future, Async, Poll};
use std::io::{self, Cursor};
use mio::Ready;

// HelloWorld has two states, namely waiting to connect to the socket
// and already connected to the socket
struct HelloWorld {
    state: HelloState,
    context: Context,
    retry_connect: u8,
    retry_loop: u8,
}

enum HelloState {
    Connecting,
    Connected,
    Receiving,
    Check,
    Respond,
    RetryConnect,
    Loop5,
}

enum HelloBranch {
    Continue,
    Notify(HelloState),
    Stop
}

enum ReadState {
    Read,
    NeedMore,
    Failed
}

struct Context {
    connect: Option<ConnectFuture>, 
    stream: Option<TcpStream>, 
    buffer: BytesMut,
}
impl HelloWorld {

    fn connecting(&mut self) -> HelloBranch {
        
        println!("connecting");

        match self.context.connect {
            None => {
                let addr = "127.0.0.1:12345".parse().unwrap();
                let stream = TcpStream::connect(&addr);
                self.context.connect = Some(stream);
            },
            _ => {} ,
        };

        match self.context.connect {
            None => {
                HelloBranch::Notify(HelloState::Connecting)
            },
            Some(ref mut connect) => {
                match connect.poll() {
                    Ok(Async::Ready(stream)) => {
                        println!("connecting [Ready]");
                        self.context.stream = Some(stream);
                        HelloBranch::Notify(HelloState::Connected)

                    },
                    Ok(Async::NotReady) => {
                        println!("connecting [NotReady]");
                        HelloBranch::Continue
                    },
                    Err(e) => { 
                        println!("connecting [Err: {}]", e);
                        HelloBranch::Notify(HelloState::RetryConnect)
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
            true => HelloState::Loop5,
            _ => HelloState::Connecting
        };

        HelloBranch::Notify(state)
    }

    fn retry_connect(&mut self) -> HelloBranch {

        println!("retry_connect");
        println!("retry_connect [retry: {}]", self.retry_connect);

        let mut result = HelloBranch::Notify(HelloState::Connecting);
        self.context.connect = None;
        self.retry_connect = self.retry_connect + 1;
        if self.retry_connect > 10 {
            result = HelloBranch::Stop;
        }
        result
    }

    fn connected(&mut self) -> HelloBranch {

        println!("connected");

        let mut data = Cursor::new(Bytes::from_static(b"hello world\n"));
        if let Some(ref mut socket) = self.context.stream {

            // Keep trying to write the buffer to the socket as long as the
            // buffer has more bytes it available for consumption
            while data.has_remaining() {

                let state = match socket.write_buf(&mut data) {
                    Ok(Async::Ready(size)) => {
                        println!("connected [Ready]");
                        HelloBranch::Notify(HelloState::Receiving)
                    },
                    Ok(Async::NotReady) => {
                        println!("connected [NotReady]");
                        HelloBranch::Notify(HelloState::Connected)
                    },
                    Err(e) => { 
                        println!("connected [Err: {}]", e);
                        HelloBranch::Notify(HelloState::RetryConnect)
                    }
                };
                return state;
            }
        }
        HelloBranch::Notify(HelloState::RetryConnect)
    }

    fn receiving(&mut self) -> HelloBranch {

        println!("receiving");
        let mut buffer = &mut self.context.buffer;
        let result2 = match self.context.stream {
            Some(ref mut socket) => {

                let mut result = ReadState::Read;
                //let read = try_ready!(socket.read_buf(&mut buffer));

                let read = match socket.read_buf(&mut buffer) {
                    Ok(Async::Ready(read)) => {
                        println!("receiving [Ready]");
                        read
                    },
                    Ok(Async::NotReady) => {
                        println!("receiving [NotReady]");
                        return HelloBranch::Continue;
                    },
                    Err(e) => { 
                        println!("receiving [Err: {}]", e);
                        return HelloBranch::Notify(HelloState::RetryConnect)
                    }
                };

                println!("receiving [read: {}]", read);
                println!("receiving [len: {}]", buffer.len());
                println!("receiving [cap: {}]", buffer.capacity());
                //println!("receiving [value: {}]", value);
                
                if read == 0 {
                    result = ReadState::Failed;
                }
                
                if read < buffer.capacity() {
                }
                
                if read == buffer.capacity() {
                    result = ReadState::NeedMore;
                }
                
                result
            }
            _ => ReadState::Failed
        };

        match result2 {
            ReadState::NeedMore => HelloBranch::Notify(HelloState::Receiving),
            ReadState::Read => HelloBranch::Notify(HelloState::Check),
            ReadState::Failed => {
                HelloBranch::Stop
            },
        }
    }

    fn check(&mut self) -> HelloBranch {

        println!("Check");

        let buffer = &mut self.context.buffer;
        let value = String::from_utf8(buffer.to_vec()).unwrap();
        let expected = "hi!\n".to_string();

        println!("Check [buffer: {}]", buffer.len());
        println!("Check [expected: {}]", expected.len());

        let mut result = HelloBranch::Notify(HelloState::Respond);
        if value == "hi!\n".to_string() {
            result =  HelloBranch::Stop;
        }

        buffer.truncate(0);
        result
    }

    fn respond(&mut self) -> HelloBranch {

        println!("respond");

        if let Some(ref mut socket) = self.context.stream {

            // Keep trying to write the buffer to the socket as long as the
            // buffer has more bytes it available for consumption
            let mut data = Cursor::new(Bytes::from_static(b"sorry\n"));
            while data.has_remaining() {
                // try_ready!(socket.write_buf(&mut data));

                let state = match socket.write_buf(&mut data) {
                    Ok(Async::Ready(size)) => {
                        println!("respond [Ready]");
                        HelloBranch::Notify(HelloState::Receiving)
                    },
                    Ok(Async::NotReady) => {
                        println!("respond [NotReady]");
                        HelloBranch::Notify(HelloState::Connected)
                    },
                    Err(e) => { 
                        println!("respond [Err: {}]", e);
                        HelloBranch::Notify(HelloState::RetryConnect)
                    }
                };
                return state;

            }
        }
        HelloBranch::Notify(HelloState::Receiving)
    }
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {

        let decision : HelloBranch = match self.state {
            HelloState::Connecting => self.connecting(),
            HelloState::Loop5 => self.loop5(),
            HelloState::RetryConnect => self.retry_connect(),
            HelloState::Connected => self.connected(),
            HelloState::Receiving => self.receiving(),
            HelloState::Check => self.check(),
            HelloState::Respond => self.respond(),
        };

        let current = futures::task::current();
        println!("poll [is_current: {}]", current.is_current());
        println!("poll [will_notify_current: {}]", current.will_notify_current());
        
        match decision {
            HelloBranch::Notify(state) => {
                self.state = state;
                futures::task::current().notify();
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

    let buffer = BytesMut::with_capacity(1024);

    let context = Context {
        connect: None, 
        stream: None,
        buffer: buffer,
    };

    let hello_world = HelloWorld {
        state: HelloState::Connecting,
        context: context,
        retry_connect: 0,
        retry_loop: 0
    };

    // Run it, here we map the error since tokio::run expects a Future<Item=(), Error=()>
    tokio::run(hello_world.map_err(|e| println!("{0}", e)))
}