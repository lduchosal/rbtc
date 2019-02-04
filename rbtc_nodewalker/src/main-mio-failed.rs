use mio::{Events, Ready, Poll, PollOpt, Token};
use mio::net::TcpStream;
use std::time::Duration;

use std::io::Read;
use std::io::Write;

const READ : Token = Token(0);
const WRITE : Token = Token(1);

fn main() {

    println!("main");
    println!("newpoll");
    let poll = Poll::new().expect("newpoll failed");
    let mut events = Events::with_capacity(5);
    
    let parse = &"127.0.0.1:12345".parse().expect("parse failed");

    loop {

        let mut stream = TcpStream::connect(parse).expect("connect failed");
        poll.register(&stream, WRITE, Ready::writable(), PollOpt::edge()).expect("register failed");
        poll.register(&stream, READ, Ready::readable(), PollOpt::edge()).expect("register failed");
        
        println!("poll");
        poll.poll(&mut events, Some(Duration::from_millis(1000))).expect("poll failed");

        println!("loop");
        for event in events.iter() {
            println!("event [event {:?}]", event);
            println!("event [readiness {:?}]", event.readiness());
            match event.token() {
                
                READ => {
                    println!("READ");
                    let mut buffer = [0u8; 128];
                    let mut count = 10;

                    let mut result: Vec<u8> = Vec::new();
                    while let Ok(read) = stream.read(&mut buffer) {
                        println!("read: {} {}", count,  read);
                        let mut temp = buffer.to_vec();
                        temp.truncate(read);
                        result.append(&mut temp);
                        if read < buffer.len() { break; }
                    }

                    let ss = String::from_utf8(result).unwrap();
                    print!("{}", ss);

                },

                WRITE => {
                    println!("WRITE");
                }
                _ => unreachable!()
            }
        }
        println!("sleep");
        std::thread::sleep_ms(1000);

        let send = &[ '.' as u8 ];
        match stream.write(send) {
            Ok(write) => println!("write: {}", write),
            Err(err) => println!("err: {}", err),
        } 
    }


    println!("end");


}