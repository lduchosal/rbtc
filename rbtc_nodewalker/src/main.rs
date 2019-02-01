use mio::*;
use mio::unix::UnixReady;
use mio::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Cursor};
// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() {

    println!("main");

    let addr = "127.0.0.1:12345".parse().unwrap();

    let mut sock = TcpStream::connect(&addr).unwrap();
    let mut events = Events::with_capacity(1024);

    let poll = Poll::new().unwrap();
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();

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

            let unix_ready = UnixReady::from(readiness);

            println!("readiness : {:#?}", unix_ready);
            println!("is_writable : {:#?}", unix_ready.is_writable());
            println!("is_readable : {:#?}", unix_ready.is_readable());
            println!("is_error : {:#?}", unix_ready.is_error());


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
                            poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();
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
                                println!("read err: {}", err.kind());
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