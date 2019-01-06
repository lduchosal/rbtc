extern crate chrono;

use rbtc::network::networkaddr::NetworkAddr;
use rbtc::network::version::Version;
use rbtc::network::version::Service;
use rbtc::network::message::{Encodable, Message, Magic};

use rbtc::utils::hexdump;

use std::net::{TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum NodeWalkerError {
    Encode,
    Connect,
    ReadTimeout,
    Read,
    Write,
    WriteTimeout,
    Shutdown
}

impl fmt::Display for NodeWalkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct NodeWalker {

}

impl NodeWalker {

    pub fn new() -> NodeWalker {
        NodeWalker {
        }
    }

    pub fn walk(&self, nodeip: &String) -> Result<Vec<String>, NodeWalkerError> {

        let mut node_ip_port = nodeip.clone();
        node_ip_port.push_str(":8333");
        let addr : SocketAddr = node_ip_port.parse().unwrap();

        println!("Connect to {}", node_ip_port);

        let payload = self.payload();
        let message = Message {
            magic: Magic::MainNet,
            payload: &payload
        };
        let mut request : Vec<u8> = Vec::new();
        message.encode(&mut request).map_err(|_| NodeWalkerError::Encode)?;
        let _content = request.as_slice();

        let mut stream = self.connect(&addr)?;
        stream.write(&request).map_err(|_| NodeWalkerError::Write)?;

        let mut response : Vec<u8> = Vec::new();
        let mut buffer = [0u8; 2048];

        loop {
            let mut read : usize = 0;
            match stream.read(&mut buffer) {
                Ok(bytes) => read = bytes,
                Err(err) => {
                    println!("Err: {:?}", err);
                    match err.kind() {
                        ErrorKind::WouldBlock => break,
                        _ => {}
                    };
                }
            };

            let mut temp = buffer.to_vec();
            temp.truncate(read);
            response.append(&mut temp);
            if read < buffer.len()
                && response.len() > 200 {
                break;
            }
        }

        // stream.shutdown(Shutdown::Both).map_err(|_| NodeWalkerError::Shutdown)?;
        let encoded  = hexdump::encode(response);
        let mut result : Vec<String> = Vec::new();
        for line in encoded.lines() {
            result.push(line.to_string());
        }
        Ok(result)
    }

    fn payload(&self) -> Version {

        let now = chrono::Local::now();

        Version {
            version: 70002,
            services: Service::Network,
            timestamp: now.timestamp(),
            receiver: NetworkAddr {
                time: None,
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            sender: NetworkAddr {
                time: None,
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            nonce: 0xE83EE8FCCF20D947,
            user_agent: "/Satoshi:0.17.0.1/".to_string(),
            start_height: 0x00049F2C,
            relay: true,
        }
    }

    fn connect(&self, addr: &SocketAddr) -> Result<TcpStream, NodeWalkerError> {

        let connect_timeout = std::time::Duration::from_secs(3);
        let read_timeout = std::time::Duration::from_secs(10);
        let write_timeout = std::time::Duration::from_secs(5);

        let stream = TcpStream::connect_timeout(addr, connect_timeout).map_err(|_| NodeWalkerError::Connect)?;
        stream.set_read_timeout(Option::Some(read_timeout)).map_err(|_| NodeWalkerError::ReadTimeout)?;
        stream.set_write_timeout(Option::Some(write_timeout)).map_err(|_| NodeWalkerError::WriteTimeout)?;

        Ok(stream)
    }
}

#[cfg(test)]
mod test {

    use crate::walker::NodeWalker;

    #[test]
    fn when_walker_walk_localhost_then_return_some_strings() {

        let walker = NodeWalker::new();
        let node = String::from("127.0.0.1");
        let result = walker.walk(&node);

        assert!(result.is_ok());

        for line in result.unwrap() {
            println!("{}", line);
        }

        
    }

    #[test]
    fn when_walker_walk_workinghost_then_return_some_strings() {

        let walker = NodeWalker::new();
        let node = String::from("142.93.2.255");
        let result = walker.walk(&node);

        assert!(result.is_ok());

        for line in result.unwrap() {
            println!("{}", line);
        }
    }
}