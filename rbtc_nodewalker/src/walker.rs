extern crate rand;

use crate::message::MessageProvider;
use crate::fsm;

use rbtc::network::message::Message;
use rbtc::network::message::Payload;
use rbtc::encode::error::Error;
use rbtc::encode::encode::{Encodable, Decodable};

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

pub struct NodeWalker {
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

    pub(crate) fn init_connect_retry(&mut self) -> InitConnectResult  {

        println!("init_connect_retry");

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

    pub(crate) fn connect_retry(&mut self) -> ConnectRetryResult {

        println!("connect_retry");

        let retry  = self.connect_retry;
        let maxretry = 1;

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

    pub(crate) fn init(&mut self) -> InitResult {

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

    pub(crate) fn connect(&mut self) -> ConnectResult {

        println!("connect");

        let addr = self.addr.unwrap();

        let connect_timeout = std::time::Duration::from_secs(3);
        let read_timeout = std::time::Duration::from_secs(3);
        let write_timeout = std::time::Duration::from_secs(3);

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

    pub(crate) fn send_version(&mut self) -> SendMessageResult {

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

    pub(crate) fn receive_version(&mut self) -> ReceiveMessageResult {

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

    pub(crate) fn receive_verack(&mut self) -> ReceiveMessageResult {

        println!("receive_verack");

        self.receive_message(|payload| {
            match payload {
                Payload::VerAck(_) => true,
                _ => false,
            }
        })
    }

    pub(crate) fn receive_addr(&mut self) -> ReceiveMessageResult {

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

    pub(crate) fn send_verack(&mut self) -> SendMessageResult {

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

    pub(crate) fn send_getaddr_retry(&mut self) -> SendGetAddrRetryResult {

        println!("send_getaddr_retry");

        let retry  = self.getaddr_retry;
        let maxretry = 2;

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

    pub(crate) fn set_version(&self) {
        println!("set_version");
    }

    pub(crate) fn parse_addr(&mut self) {

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

    pub(crate) fn end(&self) {

    }
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
