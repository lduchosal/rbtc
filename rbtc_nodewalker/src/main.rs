#[macro_use] extern crate log;
extern crate atomic;

pub mod stateless;

use mio::*;
use mio::net::TcpStream;
use std::io::{Read, Write, Cursor};
use mio::channel::Receiver;

use std::net::{SocketAddr, AddrParseError};
use std::collections::HashMap;

use stateless::StateMachine;
use stateless::Trigger;
use stateless::State;

use std::io;

struct TcpClient {
    machine: stateless::StateMachine,
    socket: Option<TcpStream>,
    addr: Option<SocketAddr>,
    registration: (mio::Registration, mio::SetReadiness),
    poll_receiver: Option<mio::channel::Receiver<String>>,
}

impl TcpClient {
    
    fn new() -> TcpClient {

        trace!("new");

        let mut m = StateMachine::new(State::Init);
        m.configure(State::Init)
            .on_entry(|| println!("-> Init"))
            .on_exit(|| println!("Init ->"))
            .permit(Trigger::ConnectFailed, State::Init)
            .permit(Trigger::ConnectSucceed, State::Connected)
            .permit(Trigger::ConnectParseAddrFailed, State::End)
            .permit(Trigger::ConnectRetryFailed, State::End)
        ;

        m.configure(State::Connected)
            .on_entry(|| println!("-> Connected"))
            .on_exit(|| println!("Connected ->"))
            .permit(Trigger::ReadSucceed, State::Connected)
            .permit(Trigger::ReadFailed, State::Connected)
            .permit(Trigger::ReadRetryFailed, State::End)
            .permit(Trigger::WriteSucceed, State::Connected)
            .permit(Trigger::WriteFailed, State::Connected)
            .permit(Trigger::WriteRetryFailed, State::End)
        ;

        m.configure(State::End)
            .on_entry(|| println!("-> End"))
            .on_exit(|| println!("End ->"))
        ;

        TcpClient {
            machine: m,
            addr: None,
            socket: None,
            registration:  Registration::new2(),
            poll_receiver: None,
        }
    }

    fn on_ready(&self, event: mio::event::Event) {
        trace!("on_ready");

    }

    fn on_readable(&self, event: mio::event::Event) {
        trace!("on_readable");

    }

    fn on_writable(&self, event: mio::event::Event) {
        trace!("on_writable");

    }

    fn on_error(&self, event: mio::event::Event) {
        trace!("on_error");

    }

}

impl Evented for TcpClient {

    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        let (registration, _readiness) = &self.registration;
        registration.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        let (registration, _readiness) = &self.registration;
        registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        let (registration, _readiness) = &self.registration;
        registration.deregister(poll)
    }
}

struct Engine {
    event_loop_sender: mio::channel::Sender<TcpClient>,
    event_loop: Option<std::thread::JoinHandle<()>>,

    message_loop_sender: mio::channel::Sender<String>,
    message_loop: Option<std::thread::JoinHandle<()>>
}

impl Engine  {

    fn new() -> Engine  {

        let (event_loop, event_loop_sender) = spawn_event_loop();
        let (message_loop, message_loop_sender) = spawn_message_loop();

        let mut engine = Engine {
            event_loop_sender: event_loop_sender,
            event_loop: Noevent_loopne,

            message_loop_sender: message_loop_sender,
            message_loop: message_loop,
        };


        engine.event_loop.replace(event_loop);
        engine.message_loop.replace(message_loop);

        engine
    }

    fn register(&mut self, mut client: TcpClient) -> TcpClientHandle {

        trace!("register");

        let (sender, receiver) = mio::channel::channel();
        client.poll_receiver.replace(receiver);

        self.clients.insert(token, client);

        TcpClientHandle::new(sender)
    }

    fn spawn_message_loop() -> (std::thread::JoinHandle<()>, mio::channel::Sender<TcpClient>) {

        trace!("spawn_message_loop");

        let (sender, receiver) = mio::channel::channel();
        let thread = std::thread::spawn(move || {

            let mut tokenid : usize = 1;
            let clients = HashMap::<Token, TcpClient>::new();

            loop {
                let client = receiver.try_recv();
                tokenid += 1;
                let token = Token(tokenid);
            }

        });

        (thread, sender)
    }

    fn spawn_event_loop() -> (std::thread::JoinHandle<()>, mio::channel::Receiver<String>) {

        trace!("spawn_event_loop");

        let (sender, receiver) = mio::channel::channel();

        let thread = std::thread::spawn(move || {
            
            let mut events = Events::with_capacity(1024);
            let mut poll = mio::Poll::new().unwrap();
            
            loop {

                println!("polling");
                poll.poll(&mut events, None).unwrap();
                println!("events : {:#?}", events.len());

                for event in events.iter() {

                    println!("event");

                    let readiness = event.readiness();
                    let token = event.token();
                    //let client = self.clients.get(&token).unwrap();

                    if readiness.is_error() {
                        sender.send("is_error".to_string());
                    }

                    if readiness.is_readable()
                        || readiness.is_writable() {
                        sender.send("on_ready".to_string());
                    }

                    if readiness.is_readable() {
                        sender.send("is_readable".to_string());
                    }

                    if readiness.is_writable() {
                        sender.send("on_writable".to_string());
                    }
                }

            }

            ()
        });

        (thread, receiver)
    }
}


struct TcpClientHandle {
    sender: mio::channel::Sender<String>,
}

impl TcpClientHandle {

    fn new(sender: mio::channel::Sender<String>) -> TcpClientHandle {
        TcpClientHandle {
            sender: sender,
        }
    }

    fn set_addr(&mut self, addr: String) {
        trace!("set_addr");
        debug!("set_addr [addr: {}]", addr);

        self.sender.send("set_addr".to_string());
    }

    fn connect(&self) {
        trace!("connect");
        self.sender.send("connect".to_string());
    }

    fn write_hello(&self) {
        trace!("write_hello");
        self.sender.send("write_hello".to_string());
    }
}


fn main() {

    println!("main");
    //let poll = mio::Poll::new().expect("Unable to start mio Poll");
    let mut engine = Engine::new();
    let tcpclient = TcpClient::new();

    let mut handle = engine.register(tcpclient);

    handle.set_addr("127.0.0.1:8333".to_string());
    handle.connect();
    handle.write_hello();

}