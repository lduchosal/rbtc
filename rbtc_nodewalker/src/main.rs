use mio::*;
use mio::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Cursor};
// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() {

    println!("main");

    let addr = "127.0.0.1:13265".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create a poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::all(),
                PollOpt::edge()).unwrap();

    // Setup the client socket
    let mut sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    poll.register(&sock, CLIENT, Ready::all(),
                PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {

                    println!("Server");
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let accept = server.accept();
                    println!("event : {:#?}", event);
                    match accept {
                        Err(err) => println!("Error with accept"),
                        Ok((mut stream, addr))  => {
                            println!("Conn accepted for {}", addr);
                            println!("Sending data");

                            stream.write("hello world".as_ref()).unwrap();
                        }
                    }
                }
                CLIENT => {
                    println!("Client");
                    println!("event : {:#?}", event);
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);

                    let size = sock.read(&mut buf).unwrap();
                    let result = String::from_utf8(buf).unwrap();
                    println!("Received: {}", result);
                    println!("Size: {}", size);

                    sock.write("hello world".as_ref()).unwrap();

                    //return;
                }
                _ => unreachable!(),
            }
        }
    }
}