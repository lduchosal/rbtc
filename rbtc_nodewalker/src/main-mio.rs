extern crate sm;
#[macro_use] extern crate log;

use sm::NoneEvent;
use sm::sm;
use std::net::SocketAddr;

use mio::*;
use mio::net::TcpStream;
use std::io::{Read, Write, Cursor};
// Setup some tokens to allow us to identify which event is
// for which socket.


const CLIENT: Token = Token(1);

fn main() {

    println!("main");

    let mut sock = TcpStream::connect(&addr).unwrap();
    let mut events = Events::with_capacity(1024);

    let poll = Poll::new().unwrap();
    poll.register(&sock, CLIENT, mio::Ready::readable(), PollOpt::edge()).unwrap();


    loop {

        println!("polling");

        poll.poll(&mut events, None).unwrap();
        println!("events : {:#?}", events.len());

        for event in events.iter() {

            println!("event");

            let readiness = event.readiness();
            println!("event : {:#?}", event);
            println!("readiness : {:#?}", readiness);
            println!("is_writable : {:#?}", readiness.is_writable());
            println!("is_readable : {:#?}", readiness.is_readable());
            println!("is_error : {:#?}", readiness.is_error());

            match event.token() {
                CLIENT => {

                    let error = sock.take_error();
                    println!("error : {:#?}", error);
                    match error {
                        Ok(Some(err)) => {
                            println!("Error occurred, sleeping 1s");
                            std::thread::sleep_ms(1000);
                            poll.deregister(&sock).unwrap();

                            sock = TcpStream::connect(&addr).unwrap();
                            poll.register(&sock, CLIENT, mio::Ready::readable(), PollOpt::edge()).unwrap();
                            continue;
                        },
                        _ => {},
                    }

                    println!("kind : {:#?}", event.kind());
                    println!("token : {:#?}", event.token());


                    loop {
                        let mut buf: Vec<u8> = vec![0u8; 256];
                        let read = sock.read(&mut buf);
                        match read {
                            Ok(size) => {
                                let result = String::from_utf8(buf).unwrap();
                                println!("read: {}", size);
                                println!("result: {}", result);
                            },
                            Err(err) => {
                                println!("read err: {}", err);
                                println!("read kind: {:#?}", err.kind());
                                break;
                            }
                        }
                    }


                    let writen = sock.write("hello world".as_ref()).unwrap();
                    println!("writen: {}", writen);

                }
                _ => unreachable!(),
            }
        }
    }
}
