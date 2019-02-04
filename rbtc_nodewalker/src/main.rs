#[macro_use] extern crate log;
extern crate atomic;

use mio::*;
use mio::net::TcpStream;
use std::io::{Read, Write, Cursor};

use std::net::{SocketAddr, AddrParseError};
use std::collections::HashMap;

struct TcpClient {
    machine: StateMachine,
    business: Business,
    addr: Option<SocketAddr>,
    socket: Option<TcpStream>,
}

impl TcpClient {
    
    fn new() -> TcpClient {

        trace!("new");

        let mut m = StateMachine::new(State::Init);
        m.configure(State::Init)
            .permit(Trigger::ConnectFailed, State::Init)
            .permit(Trigger::ConnectSucceed, State::Connected)
            .permit(Trigger::ConnectParseAddrFailed, State::End)
            .permit(Trigger::ConnectRetryFailed, State::End)
        ;

        m.configure(State::Connected)
            .permit(Trigger::ReadSucceed, State::Connected)
            .permit(Trigger::ReadFailed, State::Connected)
            .permit(Trigger::ReadRetryFailed, State::End)
        ;

        m.configure(State::Connected)
            .permit(Trigger::WriteSucceed, State::Connected)
            .permit(Trigger::WriteFailed, State::Connected)
            .permit(Trigger::WriteRetryFailed, State::End)
        ;

        TcpClient {
            machine: m,
            business: Business {},
            addr: None,
            socket: None,
        }
    }

    fn connect(&mut self, addr: String) {

        trace!("connect");
        debug!("connect [addr: {}]", addr);

        let addr = self.business.do_parse_addr(addr);
        if let Err(err) = addr {
            debug!("connect failed [err: {}]", err);
            self.machine.fire(Trigger::ConnectParseAddrFailed);
            return;
        }
        self.addr = Some(addr.unwrap());

        self.machine.fire(Trigger::ConnectSucceed);

        let socket = TcpStream::connect(&self.addr.unwrap()).unwrap();
        self.socket = Some(socket);
    }



    fn handle(&mut self, poll: &mio::Poll, event: mio::event::Event) {

        let mut socket = self.socket.as_ref().unwrap();
        let addr = self.addr.unwrap();

        let error = socket.take_error();
        println!("error : {:#?}", error);
        match error {
            Ok(Some(err)) => {
                println!("Error occurred, sleeping 1s");
                std::thread::sleep_ms(1000);
                poll.deregister(socket).unwrap();

                let socket = TcpStream::connect(&addr).unwrap();
                // poll.register(&socket, self.token, mio::Ready::readable(), PollOpt::edge()).unwrap();
                self.socket = Some(socket);

                return;
            },
            _ => {},
        }

        loop {
            let mut buf: Vec<u8> = vec![0u8; 256];
            let read = socket.read(&mut buf);
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

        let writen = socket.write("hello world".as_ref()).unwrap();
        println!("writen: {}", writen);

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

struct Business {
}

impl Business  {

    fn do_parse_addr(&self, addr: String) -> Result<SocketAddr, AddrParseError> {

        trace!("do_parse_addr");
        debug!("do_parse_addr [addr: {}]", addr);

        let mut node_ip_port = addr.clone();

        if let Ok(addr) = addr.parse() {
            return Ok(addr);
        }
        
        node_ip_port.push_str(":8333");
        node_ip_port.parse()
    }

    fn do_connect(&self, addr: SocketAddr, conn: TcpStream) {
    }
}

struct Engine<'P> {
    poll: &'P mio::Poll,
    clients: HashMap::<Token, TcpClient>,
    ids: atomic::Atomic<usize>,
}

impl<'P>  Engine<'P>  {

    fn new(poll: &'P mio::Poll) -> Engine<'P>  {

        let clients = HashMap::<Token, TcpClient>::new();

        Engine {
            poll: poll,
            clients: clients,
            ids: atomic::Atomic::new(1),
        }
    }

    fn next_id(&mut self) -> usize {
        self.ids.fetch_add(1, atomic::Ordering::Relaxed)
    }

    fn register(&mut self, mut client: TcpClient) {

        let tokenid = self.next_id();
        let token = Token(tokenid);

        client.connect("127.0.0.1:8333".to_string());
        if let Some(ref stream) = &client.socket {
            self.poll.register(stream, token, mio::Ready::readable(), PollOpt::edge()).unwrap();
        }
        
        self.clients.insert(token, client);
    }
    
    fn run(&mut self) {

        let clients = &self.clients;
        let mut events = Events::with_capacity(1024);
        
        loop {

            println!("polling");

            self.poll.poll(&mut events, None).unwrap();
            println!("events : {:#?}", events.len());

            for event in events.iter() {

                println!("event");

                let readiness = event.readiness();
                let token = event.token();

                println!("event : {:#?}", event);
                println!("readiness : {:#?}", readiness);
                println!("is_writable : {:#?}", readiness.is_writable());
                println!("is_readable : {:#?}", readiness.is_readable());
                println!("is_error : {:#?}", readiness.is_error());
        
                println!("kind : {:#?}", event.kind());
                println!("token : {:#?}", event.token());


                let client = clients.get_mut(&token);
                if client.is_none() {
                    continue;
                }
                let client = client.unwrap();


                if readiness.is_error() {
                    client.on_error(event);
                }

                if readiness.is_readable()
                    || readiness.is_writable() {
                    client.on_ready(event);
                }

                if readiness.is_readable() {
                    client.on_readable(event);
                }

                if readiness.is_writable() {
                    client.on_writable(event);
                }

            }
        }
    }

}

fn main() {

    println!("main");
    let poll = mio::Poll::new().expect("Unable to start mio Poll");
    let mut engine = Engine::new(&poll);
    let tcpclient = TcpClient::new();
    let poll = Poll::new().unwrap();

    engine.register(tcpclient);
    engine.run();

}



#[derive(Eq, PartialEq, Hash, Clone)]
enum State {
    Init,
    Connected,
    End
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Trigger {
    ConnectParseAddrFailed,
    ConnectSucceed,
    ConnectFailed,
    ConnectRetryFailed,
    ReadSucceed,
    ReadFailed,
    ReadRetryFailed,
    WriteSucceed,
    WriteFailed,
    WriteRetryFailed,
}

struct StateMachine {
    state: State,
    states: HashMap<State, StateConfigurationId>,
    configurations: HashMap<StateConfigurationId, StateConfiguration>,
    ids: atomic::Atomic<u32>,
}

struct StateConfiguration {
    triggers: HashMap<Trigger, TriggerConfiguration>,
    entry_actions: Vec<fn() -> ()>,
    exit_actions: Vec<fn() -> ()>,
}

struct StateConfigurationOperation<'M> {
    id: StateConfigurationId,
    machine: &'M mut StateMachine,
}

struct TriggerConfiguration {
    trigger: Trigger,
    destination: State,
}

type TriggerConfigurationId = u32;
type StateConfigurationId = u32;

impl StateMachine {

    fn new(initial_state: State) -> StateMachine {
        StateMachine {
            state: initial_state,
            states: HashMap::new(),
            configurations: HashMap::new(),
            ids: atomic::Atomic::new(0),
        }
    }

    fn next_id(&mut self) -> u32 {
        self.ids.fetch_add(1, atomic::Ordering::Relaxed)
    }

    fn next_state_configuration_id(&mut self) -> StateConfigurationId {
        self.next_id() as StateConfigurationId
    }

    fn next_trigger_configuration_id(&mut self) -> TriggerConfigurationId {
        self.next_id() as TriggerConfigurationId
    }

    fn configure(&mut self, state: State) -> StateConfigurationOperation {

        let id = self.next_state_configuration_id();
        let configuration = StateConfiguration {
            triggers: HashMap::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        };

        self.states.insert(state, id);
        self.configurations.insert(id, configuration);

        StateConfigurationOperation {
            id: id,
            machine: self,
        }
    }

    fn fire(&mut self, trigger: Trigger) {

        // csid : current state configurationid
        let current_state = &self.state;
        let csid = self.states.get(current_state);
        if csid.is_none() {
            return;
        }
        let csid = csid.unwrap();

        // csc : current state configuration
        let csc = self.configurations.get(csid);
        if csc.is_none() {
            return;
        }
        let csc = csc.unwrap();
        
        // cstc : current state trigger configuration
        let cstc = csc.triggers.get(&trigger);
        if cstc.is_none() {
            return;
        }
        let cstc = cstc.unwrap();

        // nsid : next state id
        let next_state = cstc.destination.clone();
        let nsid = self.states.get(&next_state);
        if nsid.is_none() {
            return;
        }
        let nsid = nsid.unwrap();

        // nsc : next state configuration
        let nsc = self.configurations.get(nsid);
        if nsc.is_none() {
            return;
        }
        let nsc = nsc.unwrap();

        for on_exit in csc.exit_actions.iter() {
            on_exit();
        }

        for on_entry in nsc.entry_actions.iter() {
            on_entry();
        }
        self.state = next_state;
    }
}

impl<'M> StateConfigurationOperation<'M> {

    fn permit(self, trigger: Trigger, destination: State) -> Self {
        let trigger_configuration = TriggerConfiguration {
            trigger: trigger.clone(),
            destination: destination,
        };

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.triggers.insert(trigger, trigger_configuration);
        self
    }

    fn on_entry(self, f: fn() -> ()) -> Self {

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.entry_actions.push(f);
        self
    }

    fn on_exit(self, f: fn() -> ()) -> Self {

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.exit_actions.push(f);
        self
    }
}
