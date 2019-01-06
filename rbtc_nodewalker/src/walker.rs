use rbtc::network::networkaddr::NetworkAddr;
use rbtc::network::version::Version;
use rbtc::network::version::Service;
use rbtc::network::message::{Encodable, Message, Magic};

use std::net::{Shutdown, TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use std::time;
use std::fmt;

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

        let mut stream = self.connect(&addr)?;
        stream.write(&request).map_err(|_| NodeWalkerError::Write)?;

        let mut response : Vec<u8> = Vec::with_capacity(2048);
        let buffer = response.as_mut_slice();
        let read = stream.read(buffer).map_err(|_| NodeWalkerError::Read)?;
        response.truncate(read);

        stream.shutdown(Shutdown::Both).map_err(|_| NodeWalkerError::Shutdown)?;
        let ss = std::str::from_utf8(&response.as_slice()).unwrap();

        let mut result : Vec<String> = Vec::new();
        result.push(ss.to_string());
        Ok(result)
    }

    fn payload(&self) -> Version {

        Version {
            version: 70002,
            services: Service::Network,
            timestamp: 1401217254,
            receiver: NetworkAddr {
                time: None,
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            sender: NetworkAddr {
                time: None,
                services: Service::Network,
                ip: IpAddr::V6("FD87:D87E:EB43:64F2:2CF5:4DCA:5941:2DB7".parse().unwrap()),
                port: 8333
            },
            nonce: 0xE83EE8FCCF20D947,
            user_agent: "/rbtc:0.2.0/".to_string(),
            start_height: 0x00049F2C,
            relay: true,
        }
    }

    fn connect(&self, addr: &SocketAddr) -> Result<TcpStream, NodeWalkerError> {

        let connect_timeout = time::Duration::from_secs(3);
        let read_timeout = time::Duration::from_secs(10);
        let write_timeout = time::Duration::from_secs(5);

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