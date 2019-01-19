extern crate mio;
extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

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
    retry_connect: u8
}

enum HelloState {
    Connecting,
    Connected,
    Receiving,
    Check,
    Respond,
    RetryConnect,
}

enum ReadState {
    Read,
    NeedMore,
    Failed
}

struct Context {
    stream: Option<TcpStream>, 
    buffer: BytesMut,
}
impl HelloWorld {
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {

        let mut i:u32 = 0;
        loop {
            i = i + 1;


            print!("{}. ", i);            
            let state = match self.state {
                HelloState::Connecting => {

                    println!("Connecting");

                    let addr = "127.0.0.1:12345".parse().unwrap();
                    let mut connect = TcpStream::connect(&addr);

                    match connect.poll() {
                        Ok(Async::Ready(stream)) => {
                            println!("Connecting [Ready]");
                            self.context.stream= Some(stream);
                            HelloState::Connected
                        },
                        Ok(Async::NotReady) => {
                            println!("Connecting [NotReady]");
                            return Ok(Async::NotReady)
                        },
                        Err(e) => { 
                            println!("Connecting [Err: {}]", e);
                            HelloState::RetryConnect
                        }
                    }
                },
                HelloState::RetryConnect => {

                    println!("RetryConnect");
                    println!("RetryConnect [retry: {}]", self.retry_connect);
                    self.retry_connect = self.retry_connect + 1;
                    if self.retry_connect > 3 {
                        return Ok(Async::Ready(()));
                    }
                    std::thread::sleep_ms(1000);
                    HelloState::Connecting
                },
                HelloState::Connected => {

                    println!("Connected");
        
                    let mut data = Cursor::new(Bytes::from_static(b"hello world\n"));
                    if let Some(ref mut socket) = self.context.stream {

                        // Keep trying to write the buffer to the socket as long as the
                        // buffer has more bytes it available for consumption
                        while data.has_remaining() {
                            try_ready!(socket.write_buf(&mut data));
                        }

                    }

                    HelloState::Receiving
                }
                HelloState::Receiving => {
                    println!("Receiving");


                    let mut buffer = &mut self.context.buffer;
                    let result2 = match self.context.stream {
                        Some(ref mut socket) => {

                            let mut result = ReadState::Read;
                            let read = try_ready!(socket.read_buf(&mut buffer));

                            println!("Received [read: {}]", read);
                            println!("Received [len: {}]", buffer.len());
                            println!("Received [cap: {}]", buffer.capacity());
                            //println!("Received [value: {}]", value);
                            
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
                        ReadState::NeedMore => HelloState::Receiving,
                        ReadState::Read => HelloState::Check,
                        ReadState::Failed => {
                            return Ok(Async::Ready(()));
                        },
                    }

                }
                HelloState::Check => {

                    println!("Check");

                    let buffer = &mut self.context.buffer;
                    let value = String::from_utf8(buffer.to_vec()).unwrap();
                    let expected = "hi!\n".to_string();

                    println!("Check [buffer: {}]", buffer.len());
                    println!("Check [expected: {}]", expected.len());

                    if value == "hi!\n".to_string() {
                        return Ok(Async::Ready(()));
                    }

                    buffer.truncate(0);

                    HelloState::Respond

                },
                HelloState::Respond => {

                    println!("Respond");
        
                    if let Some(ref mut socket) = self.context.stream {

                        // Keep trying to write the buffer to the socket as long as the
                        // buffer has more bytes it available for consumption
                        let mut data = Cursor::new(Bytes::from_static(b"sorry\n"));
                        while data.has_remaining() {
                            try_ready!(socket.write_buf(&mut data));
                        }
                    }

                    HelloState::Receiving
                }
            };
            self.state = state;
        }
    }
}


fn main() {

    
    let buffer = BytesMut::with_capacity(1024);

    let context = Context {
        stream: None,
        buffer: buffer,
    };

    let hello_world = HelloWorld {
        state: HelloState::Connecting,
        context: context,
        retry_connect: 0
    };

    // Run it, here we map the error since tokio::run expects a Future<Item=(), Error=()>
    tokio::run(hello_world.map_err(|e| println!("{0}", e)))
}