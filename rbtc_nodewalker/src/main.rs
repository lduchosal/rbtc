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

use self::RbtcFsm::Variant;
use self::RbtcFsm::Variant::*;
use self::RbtcFsm::*;

use std::collections::HashMap;


sm! {

    RbtcFsm {

        // Init
        InitialStates { Init }
        
        // Connect
        ConnectParseAddrFailed { Init => End }
        ConnectSucceed { Init => Connected }
        ConnectFailed { Init => Init }
        ConnectRetryFailed { Init => End }

        // Read
        ReadSucceed { Connected => Connected }
        ReadFailed { Connected => Connected }
        ReadRetryFailed { Connected => Init }
        
        // Write
        WriteSucceed { Connected => Connected }
        WriteFailed { Connected => Connected }
        WriteRetryFailed { Connected => Init }

    }
}
#[derive(Eq, PartialEq, Hash)]
enum State {
    Init,
    Connected,
    End
}

#[derive(Eq, PartialEq, Hash)]
enum Trigger {
    InitialStates,
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
     states: HashMap<State, StateConfiguration>,
}

struct StateConfiguration {
    triggers: Vec<TriggerConfiguration>,
    entry_actions: Vec<fn() -> ()>,
    exit_actions: Vec<fn() -> ()>,
}

struct TriggerConfiguration {
    trigger: Trigger,
    destination: State,
}

impl StateMachine {

    fn new() -> StateMachine {
        StateMachine {
            states: HashMap::new(),
        }
    }

    fn configure(&mut self, state: State) -> &StateConfiguration {
        let state_configuration = StateConfiguration {
            triggers: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        };
        self.states.insert(state, state_configuration);
        &state_configuration
    }

    fn fire(&mut self, trigger: Trigger) {
        
    }

}

impl StateConfiguration {

    fn permit(&mut self, trigger: Trigger, destination: State) -> &Self {
        let trigger_configuration = TriggerConfiguration {
            trigger: trigger,
            destination: destination,
        };
        self.triggers.push(trigger_configuration);
        &self
    }

    fn on_entry(&mut self, f: fn() -> ()) -> &Self {
        self.entry_actions.push(f);
        &self
    }

    fn on_exit(&mut self, f: fn() -> ()) -> &Self {
        self.exit_actions.push(f);
        &self
    }
}

struct TcpClient {
    id: u32,
    connect_retry: u32,
    getaddr_retry: u32,
    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,
}

impl TcpClient {
    
    fn run(&mut self) {

        trace!("run");

        let mut iteration = 0;
        let mut sm = Machine::new(Init).as_enum();

        loop {

            let sleep = std::time::Duration::from_millis(500);
            std::thread::sleep(sleep);

            debug!("run [sm: {:?}]", sm);
            debug!("run [i: {:?}]", iteration);
            debug!("run [sleep: {:?}]", sleep);

            iteration = iteration + 1;

            sm = match sm {

                // Init
                InitialInit(m) => self.on_init_by_none_event(m),
                InitByConnectFailed(m) => self.on_init_by_connect_failed(m),

                //EndByParseAddrFailed(m) => { self.on_end_by_parse_addr_failed(m); break; },

            };
        }
        debug!("thread loop ended");
    }

    fn on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>) -> Variant {
        trace!("on_init_by_none_event");
        match self.init_connect_retry() {
            InitConnectResult::Succeed => m.transition(ConnectSucceed).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ConnectParseAddrFailed).as_enum(),
            InitConnectResult::TooManyRetry => m.transition(ConnectRetryFailed).as_enum(),
        }
    }

    
    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant{
        trace!("on_init_by_connect_failed");
        match self.init_connect_retry() {
            InitConnectResult::Succeed => m.transition(ConnectSucceed).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ConnectParseAddrFailed).as_enum(),
            InitConnectResult::TooManyRetry => m.transition(ConnectRetryFailed).as_enum(),
        }
    }

    fn on_init_by_read_retry_failed(&mut self, m: Machine<Init, ReadRetryFailed>) -> Variant {
        trace!("on_init_by_read_retry_failed");
        match self.init_connect_retry() {
            InitConnectResult::Succeed => m.transition(ConnectSucceed).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ConnectParseAddrFailed).as_enum(),
            InitConnectResult::TooManyRetry => m.transition(ConnectRetryFailed).as_enum(),
        }

    }
    
    fn on_init_by_write_retry_failed(&mut self, m: Machine<Init, WriteRetryFailed>) -> Variant {
        trace!("on_init_by_none_event");
        match self.init_connect_retry() {
            InitConnectResult::Succeed => m.transition(ConnectSucceed).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ConnectParseAddrFailed).as_enum(),
            InitConnectResult::TooManyRetry => m.transition(ConnectRetryFailed).as_enum(),
        }
    }
    

    // Connected
    fn on_connected_by_connect_succeed(&mut self, m: Machine<Connected, ConnectSucceed>) -> Variant {
        trace!("on_connected_by_connect_succeed");

        match self.read_retry() {
            ReadRetryResult::Succeed => m.transition(ReadSucceed).as_enum(),
            ReadRetryResult::Failed => m.transition(ReadFailed).as_enum(),
            ReadRetryResult::TooManyRetry => m.transition(ReadRetryFailed).as_enum(),
        }
    }

    fn on_connected_by_read_succeed(&mut self, m: Machine<Connected, ReadSucceed>) -> Variant {
        trace!("on_connected_by_read_succeed");
        match self.write_retry() {
            WriteRetryResult::Succeed => m.transition(WriteSucceed).as_enum(),
            WriteRetryResult::Failed => m.transition(WriteFailed).as_enum(),
            WriteRetryResult::TooManyRetry => m.transition(WriteRetryFailed).as_enum(),
        }
    }

    fn on_connected_by_read_failed(&mut self, m: Machine<Connected, ReadFailed>) -> Variant {
        trace!("on_connected_by_read_failed");

        match self.read_retry() {
            ReadRetryResult::Succeed => m.transition(ReadSucceed).as_enum(),
            ReadRetryResult::Failed => m.transition(ReadFailed).as_enum(),
            ReadRetryResult::TooManyRetry => m.transition(ReadRetryFailed).as_enum(),
        }
    }

    fn on_connected_by_write_succeed(&mut self, m: Machine<Connected, WriteSucceed>) -> Variant {
        trace!("on_connected_by_write_succeed");
        
        match self.read_retry() {
            ReadRetryResult::Succeed => m.transition(ReadSucceed).as_enum(),
            ReadRetryResult::Failed => m.transition(ReadFailed).as_enum(),
            ReadRetryResult::TooManyRetry => m.transition(ReadRetryFailed).as_enum(),
        }
    }

    fn on_connected_by_write_failed(&mut self, m: Machine<Connected, WriteFailed>) -> Variant {
        trace!("on_connected_by_write_failed");

        match self.write_retry() {
            WriteRetryResult::Succeed => m.transition(ReadSucceed).as_enum(),
            WriteRetryResult::Failed => m.transition(ReadFailed).as_enum(),
            WriteRetryResult::TooManyRetry => m.transition(ReadRetryFailed).as_enum(),
        }
    }

    // End
    fn on_end_by_parse_addr_failed(&mut self, m: Machine<End, ConnectParseAddrFailed>) {}
    fn on_end_by_connect_retry_failed(&mut self, m: Machine<End, ConnectRetryFailed>) {}

    pub fn new(id: u32) -> TcpFsm {

        TcpFsm {
            id: id,
            connect_retry: 0,
            getaddr_retry: 0,
            addr: None,
            stream: None
        }
    }

    pub(crate) fn init_connect_retry(&mut self) -> InitConnectResult  {

        trace!("init_connect_retry");

        match self.init() {
            InitResult::ParseAddrFailed => InitConnectResult::ParseAddrFailed,
            InitResult::Succeed => {
                match self.connect_retry() {
                    ConnectRetryResult::ConnectFailed => InitConnectResult::ConnectFailed,
                    ConnectRetryResult::TooManyRetry => InitConnectResult::TooManyRetry,
                    ConnectRetryResult::Succeed => InitConnectResult::Succeed
                }
            }
        }
    }

    pub(crate) fn read_retry(&mut self) -> ReadRetryResult  {
        trace!("read_retry");
        ReadRetryResult::Succeed
    }

    pub(crate) fn write_retry(&mut self) -> WriteRetryResult  {
        trace!("write_retry");
        WriteRetryResult::Succeed
    }

    pub(crate) fn connect_retry(&mut self) -> ConnectRetryResult {

        trace!("connect_retry");

        let retry  = self.connect_retry;
        let maxretry = 1;

        debug!("connect_retry [retry: {}]", retry);
        debug!("connect_retry [maxretry: {}]", maxretry);

        self.connect_retry = self.connect_retry + 1;
        if retry >= maxretry {
            return ConnectRetryResult::TooManyRetry;
        }

        match self.connect() {
            ConnectResult::Succeed => ConnectRetryResult::Succeed,
            ConnectResult::ConnectFailed => ConnectRetryResult::ConnectFailed
        }
    }

    pub(crate) fn init(&mut self) -> InitResult {

        trace!("init");
        debug!("init [node_ip_port: {}]", self.node_ip_port);

        let mut node_ip_port = self.node_ip_port.clone();

        if let Ok(addr) = node_ip_port.parse() {
            self.addr = Some(addr);
            return InitResult::Succeed;
        }
        
        node_ip_port.push_str(":8333");
        match node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                InitResult::Succeed
            },
            Err(err) => {
                warn!("init [err: {}]", err);
                warn!("init [node_ip_port: {}]", node_ip_port);
                InitResult::ParseAddrFailed
            }
        }
    }

    pub(crate) fn connect(&mut self) -> ConnectResult {

        trace!("connect");

        let addr = self.addr.unwrap();

        let connect_timeout = std::time::Duration::from_secs(3);
        let read_timeout = std::time::Duration::from_secs(3);
        let write_timeout = std::time::Duration::from_secs(3);

        debug!("connect [connect_timeout: {:?}]", connect_timeout);
        debug!("connect [read_timeout: {:?}]", read_timeout);
        debug!("connect [write_timeout: {:?}]", write_timeout);

        match TcpStream::connect_timeout(&addr, connect_timeout) {
            Err(err) => {
                debug!("connect failed [err: {}]", err);
                self.stream = None;
                ConnectResult::ConnectFailed
            },
            Ok(stream) => {
                if let Err(err) = stream.set_read_timeout(Option::Some(read_timeout)) {
                    warn!("connect set_read_timeout [err: {}]", err);
                };
                if let Err(err) = stream.set_write_timeout(Option::Some(write_timeout)) {
                    warn!("connect set_write_timeout [err: {}]", err);
                };
                self.stream = Some(stream);
                ConnectResult::Succeed
            }
        }
    }


    fn send(&mut self, messages: Vec<Message>) -> SendResult {

        trace!("send");

        let mut stream = self.stream.as_ref().unwrap();
        let mut request : Vec<u8> = Vec::new();

        if let Err(err) = messages.encode(&mut request) {
            debug!("send messages.encode [err: {:?}]", err);
            return SendResult::EncodeFailed;
        }

        if let Err(err) = stream.write(&request) {
            debug!("send stream.write [err: {:?}]", err);
            return SendResult::WriteFailed;
        }
        
        SendResult::Succeed
    }

    fn receive(&mut self) -> ReceiveResult {

        trace!("receive");

        let mut stream = self.stream.as_ref().unwrap();
        let response = &mut self.response;
        
        let mut buffer = [0u8; 8192];
        let mut _read : usize = 0;

        debug!("receive [buffer: {}]", buffer.len());

        loop {

            match stream.read(&mut buffer) {
                Ok(bytes) => _read = bytes,
                Err(err) => {
                    debug!("receive stream.read [err: {:?}]", err);
                    return ReceiveResult::ReadFailed;
                }
            };

            debug!("receive [read: {}]", _read);
            if _read == 0 {
                return ReceiveResult::ReadEmpty;
            }

            let mut temp = buffer.to_vec();
            temp.truncate(_read);
            response.append(&mut temp);

            if _read == buffer.len() {
                continue;
            }
            // else if read < buffer.len() 
            return ReceiveResult::ReadSome;
        }

    }

}


const CLIENT: Token = Token(1);

fn main() {

    println!("main");

    let tcpfsm = TcpFsm::new(0);
    tcpfsm.run();
    

    tcpfsm.connect("127.0.0.1:12345".to_string());

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



#[derive(Debug, Clone)]
pub enum EndResult {
    ParseAddrFailed,
    RetryFailed,
    SendVersionFailed,
    ReceiveVersionFailed,
    ReceiveVerackFailed,
    SendVerackFailed,
    SendGetAddrRetryFailed,
    ParseAddr,
}

pub enum ConnectResult {
    Succeed,
    ConnectFailed,
}

pub enum InitResult {
    Succeed,
    ParseAddrFailed,
}

pub enum InitConnectResult {
    Succeed,
    ParseAddrFailed,
    ConnectFailed,
    TooManyRetry,
}

pub enum ReadRetryResult {
    Succeed,
    Failed,
    TooManyRetry,
}

pub enum WriteRetryResult {
    Succeed,
    Failed,
    TooManyRetry,
}

pub enum SendResult {
    Succeed,
    EncodeFailed,
    WriteFailed,
}

pub enum SendMessageResult {
    Succeed,
    Failed,
}

pub enum ConnectRetryResult {
    Succeed,
    ConnectFailed,
    TooManyRetry,
}

pub enum ReceiveResult {
    ReadFailed,
    ReadEmpty,
    ReadSome,
}

pub enum DecodeResult {
    NeedMoreData,
    DecodeFailed,
    Succeed
}

pub enum ReceiveMessageResult {
    Failed,
    Succeed
}

pub enum SendGetAddrRetryResult {
    TooManyRetry,
    Succeed,
    Failed
}
