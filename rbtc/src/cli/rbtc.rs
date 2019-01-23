use crate::message::MessageProvider;
use crate::walker::result::*;

use rbtc::network::message::Message;
use rbtc::network::message::Payload;
use rbtc::encode::error::Error;
use rbtc::encode::encode::{Encodable, Decodable};

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

pub struct Rbtc {
    id: u32,
    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    ips: Vec<String>,
    stream: Option<TcpStream>,
    response: Vec<u8>,
    messages: Vec<Message>,

    result: Option<EndResult>,
}

impl Rbtc {

    pub fn new(id: u32, nodeip: &String) -> Rbtc {

        let node_ip_port = nodeip.clone();
        let response = Vec::new();
        let messages = Vec::new();
        let ips: Vec<String>= Vec::new();

        Rbtc {
            id: id,
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            ips: ips,
            stream: None,
            response: response,
            messages: messages,
            result: None,
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


    pub(crate) fn end(&mut self, result: EndResult) {
        trace!("end");
        debug!("end [{:?}]", result);
        self.result = Some(result);
    }
}
