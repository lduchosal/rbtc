extern crate sm;
extern crate rand;

use crate::message::MessageProvider;

use rbtc::network::message::Message;
use rbtc::network::message::Payload;
use rbtc::encode::error::Error;
use rbtc::encode::encode::{Encodable, Decodable};

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

use self::WalkerSm::*;
use self::WalkerSm::Variant::*;

pub struct NodeWalker {
    iteration: u32,

    node_ip_port: String,
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    ips: Vec<String>,
    stream: Option<TcpStream>,
    response: Vec<u8>,
    messages: Vec<Message>,
}

impl NodeWalker {

    pub fn new(nodeip: &String) -> NodeWalker {

        let mut node_ip_port = nodeip.clone();
        if ! nodeip.ends_with(":8333") {
            node_ip_port.push_str(":8333");
        }
        let response = Vec::new();
        let messages = Vec::new();
        let ips: Vec<String>= Vec::new();

        NodeWalker {
            iteration: 0,
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            ips: ips,
            stream: None,
            response: response,
            messages: messages,
        }
    }

    pub fn ips(&self) -> Vec<String> {
        self.ips.clone()
    }

    pub fn run(&mut self) {

        println!("run");

        let mut sm = Machine::new(Init).as_enum();

        loop {
            let onesec = time::Duration::from_secs(1);
            thread::sleep(onesec);

            println!("{} {:?}", self.iteration, sm);
            self.iteration = self.iteration + 1;

            sm = match sm {
                InitialInit(m) => self.on_initial_init(m),
                ConnectByConnectSocket(m) => self.on_connect_by_connect_socket(m),
                InitByConnectFailed(m) => self.on_init_by_connect_failed(m),
                VersionSentBySendVersion(m) => self.on_version_sent_by_send_version(m),
                VersionReceivedByReceiveVersion(m) => self.on_version_received_by_receive_version(m),
                VerackReceivedByReceiveVerack(m) => self.on_verack_received_by_receive_verack(m),
                VerackSentBySendVerack(m) => self.on_verack_sent_by_send_verack(m),
                HandshakeBySetVersion(m) => self.on_handshake_by_set_version(m),
                HandshakeByReceiveOther(m) => self.on_handshake_by_receive_other(m),
                HandshakeBySendGetAddrFailed (m) => self.on_handshake_by_send_getaddr_failed(m),
                GetAddrBySendGetAddr(m) => self.on_get_addr_by_send_getaddr(m),
                AddrByReceiveAddr(m) => { self.on_addr_by_receive_addr(m); break; },
                EndByParseAddrFailed(m) => { self.on_end_by_parse_addr_failed(m); break; },
                EndByRetryFailed(m) => { self.on_end_by_retry_failed(m); break; },
                EndBySendVersionFailed(m) => { self.on_end_by_send_version_failed(m); break; },
                EndByReceiveVersionFailed(m) => { self.on_end_by_receive_version_failed(m); break; },
                EndByReceiveVerackFailed(m) => { self.on_end_by_receive_verack_failed(m); break; },
                EndBySendVerackFailed(m) => { self.on_end_by_send_verack_failed(m); break; },
                EndBySendGetAddrRetryFailed(m) => { self.on_end_by_send_get_addr_retry_failed(m); break; },
            };
        }
    }

    fn init_connect(&mut self) -> InitConnectResult  {

        println!("init_connect");

        match self.init() {
            InitResult::ParseAddrFailed => InitConnectResult::ParseAddrFailed,
            InitResult::Succeed => {
                match self.connect() {
                    ConnectResult::ConnectFailed => InitConnectResult::ConnectFailed,
                    ConnectResult::Succeed => InitConnectResult::Succeed
                }
            }
        }
    }

    fn connect_retry(&mut self) -> ConnectRetryResult {

        println!("connect_retry");

        let retry  = self.connect_retry;
        let maxretry = 3;

        println!("connect_retry [retry: {}]", retry);
        println!("connect_retry [maxretry: {}]", maxretry);

        self.connect_retry = self.connect_retry + 1;
        if retry >= maxretry {
            return ConnectRetryResult::TooManyRetry;
        }

        match self.connect() {
            ConnectResult::Succeed => ConnectRetryResult::Succeed,
            ConnectResult::ConnectFailed => ConnectRetryResult::ConnectFailed
        }
    }

    fn init(&mut self) -> InitResult {

        println!("init");
        println!("init [node_ip_port: {}]", self.node_ip_port);

        match self.node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                InitResult::Succeed
            },
            Err(err) => {
                println!("init failed [err: {}]", err);
                InitResult::ParseAddrFailed
            }
        }
    }

    fn connect(&mut self) -> ConnectResult {

        println!("connect");

        let addr = self.addr.unwrap();

        let connect_timeout = std::time::Duration::from_secs(3);
        let read_timeout = std::time::Duration::from_secs(30);
        let write_timeout = std::time::Duration::from_secs(5);

        println!("connect [connect_timeout: {:?}]", connect_timeout);
        println!("connect [read_timeout: {:?}]", read_timeout);
        println!("connect [write_timeout: {:?}]", write_timeout);

        match TcpStream::connect_timeout(&addr, connect_timeout) {
            Err(err) => {
                println!("connect failed [err: {}]", err);
                self.stream = None;
                ConnectResult::ConnectFailed
            },
            Ok(stream) => {
                if let Err(err) = stream.set_read_timeout(Option::Some(read_timeout)) {
                    println!("connect set_read_timeout [err: {}]", err);
                };
                if let Err(err) = stream.set_write_timeout(Option::Some(write_timeout)) {
                    println!("connect set_write_timeout [err: {}]", err);
                };
                self.stream = Some(stream);
                ConnectResult::Succeed
            }
        }
    }

    fn send_version(&mut self) -> SendMessageResult {

        println!("send_version");
        let messages = MessageProvider::version();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => SendMessageResult::Failed
        }
    }

    fn send(&mut self, messages: Vec<Message>) -> SendResult {

        println!("send");

        let mut stream = self.stream.as_ref().unwrap();
        let mut request : Vec<u8> = Vec::new();

        if let Err(err) = messages.encode(&mut request) {
            println!("send messages.encode [err: {:?}]", err);
            return SendResult::EncodeFailed;
        }

        if let Err(err) = stream.write(&request) {
            println!("send stream.write [err: {:?}]", err);
            return SendResult::WriteFailed;
        }
        
        SendResult::Succeed
    }

    fn receive_version(&mut self) -> ReceiveMessageResult {

        println!("receive_version");

        self.receive_message(|payload| {
            match payload {
                Payload::Version(_) => true,
                _ => false,
            }
        })
    }

    fn receive_message(&mut self, match_payload: impl Fn(&Payload) -> bool) -> ReceiveMessageResult{

        println!("receive_message");

        let messages = &self.messages;
        for message in messages {
            if match_payload(&message.payload) {
                return ReceiveMessageResult::Succeed;
            }
        }

        self.receive_decode_loop();

        let messages = &self.messages;
        for message in messages {
            if match_payload(&message.payload) {
                return ReceiveMessageResult::Succeed;
            }
        }

        ReceiveMessageResult::Failed
    }

    fn receive_verack(&mut self) -> ReceiveMessageResult {

        println!("receive_verack");

        self.receive_message(|payload| {
            match payload {
                Payload::VerAck(_) => true,
                _ => false,
            }
        })
    }

    fn receive_addr(&mut self) -> ReceiveMessageResult {

        println!("receive_addr");

        self.receive_message(|payload| {
            match payload {
                Payload::Addr(_) => true,
                _ => false,
            }
        })
    }
    

    fn decode(&mut self) -> DecodeResult {

        println!("decode");

        let response = &self.response;
        let result = &mut self.messages;
        let mut cursor = Cursor::new(response);

        match <Vec<Message>>::decode(&mut cursor) {

            Ok(mut messages) => {
                result.append(&mut messages);
                &mut self.response.clear();
                DecodeResult::Succeed
            },

            Err(err) => {

                println!("decode [{:?}]", err);

                match err {
                    Error::PayloadData => DecodeResult::NeedMoreData,
                    Error::PayloadLen => DecodeResult::NeedMoreData,
                    _ => DecodeResult::DecodeFailed,
                }
            }
        }
    }

    fn receive_decode_loop(&mut self) {

        println!("receive_loop");

        let maxloop = 5;
        let mut count = 0;

        println!("receive_loop [maxloop: {}]", maxloop);

        while count <= maxloop {
            count = count + 1;

            println!("receive_loop [count: {}]", count);

            match self.receive() {
                ReceiveResult::ReadFailed => continue,
                ReceiveResult::ReadEmpty => continue,
                _ => {},
            };

            match self.decode() {
                DecodeResult::NeedMoreData => continue,
                DecodeResult::DecodeFailed => break,
                DecodeResult::Succeed => break,
            }
        }
    }

    fn receive(&mut self) -> ReceiveResult {

        println!("receive");

        let mut stream = self.stream.as_ref().unwrap();
        let response = &mut self.response;
        
        let mut buffer = [0u8; 8192];
        let mut _read : usize = 0;

        loop {

            println!("receive [buffer: {}]", buffer.len());

            match stream.read(&mut buffer) {
                Ok(bytes) => _read = bytes,
                Err(err) => {
                    println!("receive stream.read [err: {:?}]", err);
                    return ReceiveResult::ReadFailed;
                }
            };

            println!("receive [read: {}]", _read);
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

    fn send_verack(&mut self) -> SendMessageResult {

        println!("send_verack");

        let messages = MessageProvider::verack();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => SendMessageResult::Failed
        }
    }

    fn send_getaddr(&mut self) -> SendMessageResult {

        println!("send_getaddr");

        let messages = MessageProvider::getaddr();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => SendMessageResult::Failed
        }
    }

    fn send_getaddr_retry(&mut self) -> SendGetAddrRetryResult {

        println!("send_getaddr_retry");

        let retry  = self.getaddr_retry;
        let maxretry = 3;

        println!("send_getaddr_retry [retry: {}]", retry);
        println!("send_getaddr_retry [maxretry: {}]", maxretry);

        self.getaddr_retry = self.getaddr_retry + 1;
        if retry >= maxretry {
            return SendGetAddrRetryResult::TooManyRetry;
        }

        match self.send_getaddr() {
            SendMessageResult::Succeed => SendGetAddrRetryResult::Succeed,
            _ => SendGetAddrRetryResult::Failed
        }
    }

    fn set_version(&self) {
        println!("set_version");
    }

    fn parse_addr(&mut self) {

        println!("parse_addr");
        let result = &mut self.ips;
        for message in &self.messages {
            if let Payload::Addr(addr) = &message.payload {
                for a in &addr.addrs {
                    let ip_port = format!("{}:{}", a.addr.ip, a.addr.port);
                    result.push(ip_port);
                }
            }
        }
    }

    fn end(&self) {

    }
}

enum ConnectResult {
    Succeed,
    ConnectFailed,
}

enum InitResult {
    Succeed,
    ParseAddrFailed,
}

enum InitConnectResult {
    Succeed,
    ParseAddrFailed,
    ConnectFailed,
}

enum SendResult {
    Succeed,
    EncodeFailed,
    WriteFailed,
}

enum SendMessageResult {
    Succeed,
    Failed,
}

enum ConnectRetryResult {
    Succeed,
    ConnectFailed,
    TooManyRetry,
}

enum ReceiveResult {
    ReadFailed,
    ReadEmpty,
    ReadSome,
}

enum DecodeResult {
    NeedMoreData,
    DecodeFailed,
    Succeed
}

enum ReceiveMessageResult {
    Failed,
    Succeed
}

enum SendGetAddrRetryResult {
    TooManyRetry,
    Succeed,
    Failed
}

impl WalkerSmEvents for NodeWalker  {

    fn on_initial_init(&mut self, m: Machine<Init, NoneEvent>) -> Variant {

        println!("on_initial_init");
        match self.init_connect() {
            InitConnectResult::Succeed => m.transition(ConnectSocket).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ParseAddrFailed).as_enum(),
        }
    }

    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant {

        println!("on_init_by_connect_failed");
        match self.connect_retry() {
            ConnectRetryResult::Succeed => m.transition(ConnectSocket).as_enum(),
            ConnectRetryResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            ConnectRetryResult::TooManyRetry => m.transition(RetryFailed).as_enum(),
        }
    }

    fn on_connect_by_connect_socket(&mut self, m: Machine<Connect, ConnectSocket>) -> Variant {

        println!("on_connect_by_connect_socket");
        match self.send_version() {
            SendMessageResult::Succeed => m.transition(SendVersion).as_enum(),
            _ => m.transition(SendVersionFailed).as_enum(),
        }
    }

    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant {

        println!("on_version_sent_by_send_version");
        match self.receive_version() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVersion).as_enum(),
            _ => m.transition(ReceiveVersionFailed).as_enum(),
        }
    }

    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant {
        
        println!("on_version_received_by_receive_version");
        match self.receive_verack() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVerack).as_enum(),
            _ => m.transition(ReceiveVerackFailed).as_enum(),
        }
    }

    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant {

        println!("on_verack_received_by_receive_verack");
        match self.send_verack() {
            SendMessageResult::Succeed => m.transition(SendVerack).as_enum(),
            _ => m.transition(SendVerackFailed).as_enum(),
        }
    }

    fn on_verack_sent_by_send_verack(&mut self, m: Machine<VerackSent, SendVerack>) -> Variant {

        println!("on_verack_sent_by_send_verack");
        self.set_version();
        m.transition(SetVersion).as_enum()
    }

    fn on_handshake_by_set_version(&mut self, m: Machine<Handshake, SetVersion>) -> Variant {

        println!("on_handshake_by_set_version");
        match self.send_getaddr() {
            SendMessageResult::Succeed => m.transition(SendGetAddr).as_enum(),
            _ => m.transition(SendGetAddrFailed).as_enum(),
        }
    }

    fn on_handshake_by_receive_other(&mut self, m: Machine<Handshake, ReceiveOther>) -> Variant {

        println!("on_handshake_by_receive_other");
        match self.send_getaddr_retry() {
            SendGetAddrRetryResult::Succeed => m.transition(SendGetAddr).as_enum(),
            SendGetAddrRetryResult::Failed => m.transition(SendGetAddrFailed).as_enum(),
            SendGetAddrRetryResult::TooManyRetry => m.transition(SendGetAddrRetryFailed).as_enum(),
        }
    }

    fn on_handshake_by_send_getaddr_failed(&mut self, m: Machine<Handshake, SendGetAddrFailed>) -> Variant {

        println!("on_handshake_by_receive_other");
        match self.send_getaddr_retry() {
            SendGetAddrRetryResult::Succeed => m.transition(SendGetAddr).as_enum(),
            _ => m.transition(SendGetAddrFailed).as_enum(),
        }
    }

    fn on_get_addr_by_send_getaddr(&mut self, m: Machine<GetAddr, SendGetAddr>) -> Variant {

        println!("on_get_addr_by_send_addr");
        match self.receive_addr() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveAddr).as_enum(),
            _ => m.transition(ReceiveOther).as_enum(),
        }
    }

    fn on_addr_by_receive_addr(&mut self, _m: Machine<Addr, ReceiveAddr>) {

        println!("on_addr_by_receive_addr");
        self.parse_addr();
        self.end();
    }

    fn on_end_by_parse_addr_failed(&self, _m: Machine<End, ParseAddrFailed>) {

        println!("on_end_by_parse_addr_failed");
        self.end();
    }

    fn on_end_by_retry_failed(&self, _m: Machine<End, RetryFailed>) {

        println!("on_end_by_retry_failed");
        self.end();
    }

    fn on_end_by_send_version_failed(&self, _m: Machine<End, SendVersionFailed>) {

        println!("on_end_by_send_version_failed");
        self.end();
    }

    fn on_end_by_receive_version_failed(&self, _m: Machine<End, ReceiveVersionFailed>) {

        println!("on_end_by_receive_version_failed");
        self.end();
    }

    fn on_end_by_receive_verack_failed(&self, _m: Machine<End, ReceiveVerackFailed>) {

        println!("on_end_by_receive_verack_failed");
        self.end();
    }

    fn on_end_by_send_verack_failed(&self, _m: Machine<End, SendVerackFailed>) {

        println!("on_end_by_send_verack_failed");
        self.end();
    }

    fn on_end_by_send_get_addr_retry_failed(&self, _m: Machine<End, SendGetAddrRetryFailed>) {

        println!("on_end_by_send_verack_failed");
        self.end();
    }
}

use sm::NoneEvent;
use sm::sm;
use std::{thread, time};

sm! {

    WalkerSm {

        InitialStates { Init }
        ParseAddrFailed { Init => End }
        RetryFailed { Init => End }
        ConnectSocket { Init => Connect }
        ConnectFailed { Init => Init }

        SendVersion { Connect => VersionSent }
        SendVersionFailed { Connect => End }

        ReceiveVersion { VersionSent => VersionReceived }
        ReceiveVersionFailed { VersionSent => End }

        ReceiveVerack { VersionReceived => VerackReceived }
        ReceiveVerackFailed { VersionReceived => End }

        SendVerack { VerackReceived => VerackSent }
        SendVerackFailed { VerackReceived => End }

        SetVersion { VerackSent => Handshake }

        SendGetAddr { Handshake => GetAddr }
        SendGetAddrFailed { Handshake => Handshake }
        SendGetAddrRetryFailed { Handshake => End }

        ReceiveAddr { GetAddr => Addr }
        ReceiveOther { Handshake, GetAddr => Handshake }

    }
}

trait WalkerSmEvents {
    fn on_initial_init(&mut self, m: Machine<Init, NoneEvent>) -> Variant ;
    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant ;
    fn on_connect_by_connect_socket(&mut self, m: Machine<Connect, ConnectSocket>) -> Variant ;
    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant ;
    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant ;
    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant ;
    fn on_verack_sent_by_send_verack(&mut self, m: Machine<VerackSent, SendVerack>) -> Variant ;
    fn on_handshake_by_set_version(&mut self, m: Machine<Handshake, SetVersion>) -> Variant ;
    fn on_handshake_by_receive_other(&mut self, m: Machine<Handshake, ReceiveOther>) -> Variant ;
    fn on_handshake_by_send_getaddr_failed(&mut self, m: Machine<Handshake, SendGetAddrFailed>) -> Variant ;
    fn on_get_addr_by_send_getaddr(&mut self, m: Machine<GetAddr, SendGetAddr>) -> Variant ;
    fn on_addr_by_receive_addr(&mut self, m: Machine<Addr, ReceiveAddr>);
    fn on_end_by_parse_addr_failed(&self, m: Machine<End, ParseAddrFailed>);
    fn on_end_by_retry_failed(&self, m: Machine<End, RetryFailed>);
    fn on_end_by_send_version_failed(&self, m: Machine<End, SendVersionFailed>);
    fn on_end_by_receive_version_failed(&self, m: Machine<End, ReceiveVersionFailed>);
    fn on_end_by_receive_verack_failed(&self, m: Machine<End, ReceiveVerackFailed>);
    fn on_end_by_send_verack_failed(&self, m: Machine<End, SendVerackFailed>);
    fn on_end_by_send_get_addr_retry_failed(&self, m: Machine<End, SendGetAddrRetryFailed>);
    
}

