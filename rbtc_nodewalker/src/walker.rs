
use crate::message::MessageProvider;

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

        let node_ip_port = nodeip.clone();
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

    pub(crate) fn connect_retry(&mut self) -> ConnectRetryResult {

        trace!("connect_retry");

        let retry  = self.connect_retry;
        let maxretry = 1;

        debug!("connect_retry [retry: {}]", retry);
        debug!("connect_retry [maxretry: {}]", maxretry);

        self.connect_retry = self.connect_retry + 1;
        if retry >= maxretry {
            warn!("connect_retry [TooManyRetry]");
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

    pub(crate) fn send_version(&mut self) -> SendMessageResult {

        trace!("send_version");
        
        let messages = MessageProvider::version();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => {
                warn!("send_version [Failed]");
                SendMessageResult::Failed
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

    pub(crate) fn receive_version(&mut self) -> ReceiveMessageResult {

        trace!("receive_version");

        self.receive_message(|payload| {
            match payload {
                Payload::Version(_) => true,
                _ => false,
            }
        })
    }

    fn receive_message(&mut self, match_payload: impl Fn(&Payload) -> bool) -> ReceiveMessageResult{

        trace!("receive_message");

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

        trace!("receive_verack");

        self.receive_message(|payload| {
            match payload {
                Payload::VerAck(_) => true,
                _ => false,
            }
        })
    }

    pub(crate) fn receive_addr(&mut self) -> ReceiveMessageResult {

        trace!("receive_addr");

        self.receive_message(|payload| {
            match payload {
                Payload::Addr(_) => true,
                _ => false,
            }
        })
    }
    

    fn decode(&mut self) -> DecodeResult {

        trace!("decode");

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

                debug!("decode [err: {:?}]", err);
                match err {
                    Error::PayloadCommandString => DecodeResult::NeedMoreData,
                    Error::PayloadLen => DecodeResult::NeedMoreData,
                    Error::PayloadTooSmall => DecodeResult::NeedMoreData,
                    Error::PayloadChecksum => DecodeResult::NeedMoreData,
                    Error::PayloadData => DecodeResult::NeedMoreData,
                    _ => DecodeResult::DecodeFailed,
                }
            }
        }
    }

    fn receive_decode_loop(&mut self) {

        trace!("receive_decode_loop");

        let loop_max = 20;
        let mut loop_count = 0;
        
        let error_max = 5;
        let mut error_count = 0;

        debug!("receive_decode_loop [loop_max: {}]", loop_max);
        debug!("receive_decode_loop [error_max: {}]", error_max);

        while loop_count <= loop_max
              && error_count <= error_max
        {
            loop_count = loop_count + 1;

            debug!("receive_decode_loop [loop_count: {}]", loop_count);
            debug!("receive_decode_loop [error_count: {}]", error_count);

            match self.receive() {
                ReceiveResult::ReadFailed => {
                    error_count = error_count + 1;
                    continue
                },
                ReceiveResult::ReadEmpty => {
                    error_count = error_count + 1;
                    continue
                },
                ReceiveResult::ReadSome => {
                    // dont break nor continue, try decode below
                }, 
            };

            match self.decode() {
                DecodeResult::NeedMoreData => continue,
                DecodeResult::DecodeFailed => break,
                DecodeResult::Succeed => break,
            }
        }
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

    pub(crate) fn send_verack(&mut self) -> SendMessageResult {

        trace!("send_verack");

        let messages = MessageProvider::verack();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => SendMessageResult::Failed
        }
    }

    fn send_getaddr(&mut self) -> SendMessageResult {

        trace!("send_getaddr");

        let messages = MessageProvider::getaddr();
        match self.send(messages) {
            SendResult::Succeed => SendMessageResult::Succeed,
            _ => SendMessageResult::Failed
        }
    }

    pub(crate) fn send_getaddr_retry(&mut self) -> SendGetAddrRetryResult {

        trace!("send_getaddr_retry");

        let retry  = self.getaddr_retry;
        let maxretry = 2;

        debug!("send_getaddr_retry [retry: {}]", retry);
        debug!("send_getaddr_retry [maxretry: {}]", maxretry);

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
        trace!("set_version");
    }

    pub(crate) fn parse_addr(&mut self) {

        trace!("parse_addr");
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
        trace!("end");
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
