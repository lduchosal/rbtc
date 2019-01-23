use crate::cli::result::*;

use crate::network::message::Message;
use crate::network::message::Payload;
use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

pub struct Rbtc {
    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,
    response: Vec<u8>,
}

impl Rbtc {

    pub fn new() -> Rbtc {

        let response = Vec::new();
        let node_ip_port = "127.0.0.1:8333".to_string();

        Rbtc {
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            stream: None,
            response: response,
        }
    }

    pub(crate) fn init(&mut self) -> InitResult {
        trace!("init");
        InitResult::Succeed
    }

    pub(crate) fn set_addr(&mut self) -> SetAddrResult {

        trace!("set_addr");
        debug!("set_addr [node_ip_port: {}]", self.node_ip_port);

        let mut node_ip_port = self.node_ip_port.clone();

        if let Ok(addr) = node_ip_port.parse() {
            self.addr = Some(addr);
            return SetAddrResult::Succeed;
        }
        
        node_ip_port.push_str(":8333");
        match node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                SetAddrResult::Succeed
            },
            Err(err) => {
                warn!("set_addr [err: {}]", err);
                warn!("set_addr [node_ip_port: {}]", node_ip_port);
                SetAddrResult::ParseAddrFailed
            }
        }
    }

}
