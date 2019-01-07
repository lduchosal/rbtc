use rbtc::network::getaddr::GetAddr;
use rbtc::network::networkaddr::NetworkAddr;
use rbtc::network::version::Version;
use rbtc::network::version::Service;
use rbtc::network::message::{Encodable, Message, Magic};

use std::net::{Shutdown, TcpStream, IpAddr};
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

    /// 
    /// https://en.bitcoin.it/wiki/Version_Handshake
    /// 
    /// On connect, version and verack messages are exchanged, in order to ensure compatibility between peers.
    /// 
    /// Version Handshake
    /// When the local peer (L) connects to a remote peer (R), the remote peer will not send any data until it receives a version message.
    /// 
    /// L -> R: Send version message with the local peer's version
    /// R -> L: Send version message back
    /// R -> L: Send verack message
    /// R:      Sets version to the minimum of the 2 versions
    /// L -> R: Send verack message after receiving version message from R
    /// L:      Sets version to the minimum of the 2 versions
    /// 
    /// Note: Versions below 31800 are no longer supported.
    /// 
    pub fn walk(&self, nodeip: &String) -> Result<Vec<String>, NodeWalkerError> {

        let mut node_ip_port = nodeip.clone();
        node_ip_port.push_str(":8333");
        let addr = node_ip_port.parse().unwrap();

        println!("Connect to {}", node_ip_port);

        // let message = Message {
        //     magic: Magic::MainNet,
        //     payload: &GetAddr {}
        // };

        let message = Message {
            magic: Magic::MainNet,
            
            payload: &Version {
                version: 70002,
                services: Service::Network,
                timestamp: 1401217254,
                receiver: NetworkAddr {
                    time: None,
                    services: Service::Network,
                    ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                    port: 0
                }, //[ 1, 0, 0, 0, 0, 0, 0, 0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ],
                sender: NetworkAddr {
                    time: None,
                    services: Service::Network,
                    ip: IpAddr::V6("FD87:D87E:EB43:64F2:2CF5:4DCA:5941:2DB7".parse().unwrap()),
                    port: 8333
                }, //  [ 1, 0, 0, 0, 0, 0, 0, 0, 0xfd, 0x87, 0xd8, 0x7e, 0xeb, 0x43, 0x64, 0xf2, 0x2c, 0xf5, 0x4d, 0xca, 0x59, 0x41, 0x2d, 0xb7, 0x20, 0x8d ],
                nonce: 0xE83EE8FCCF20D947,
                user_agent: "/Satoshi:0.9.99/".to_string(),
                start_height: 0x00049F2C,
                relay: true,
            }
        };

        let mut request : Vec<u8> = Vec::new();
        message.encode(&mut request).map_err(|_| NodeWalkerError::Encode)?;

        let connect_timeout = time::Duration::from_secs(3);
        let read_timeout = time::Duration::from_secs(10);
        let write_timeout = time::Duration::from_secs(5);

        let mut stream = TcpStream::connect_timeout(&addr, connect_timeout).map_err(|_| NodeWalkerError::Connect)?;
        stream.set_read_timeout(Option::Some(read_timeout)).map_err(|_| NodeWalkerError::ReadTimeout)?;
        stream.set_write_timeout(Option::Some(write_timeout)).map_err(|_| NodeWalkerError::WriteTimeout)?;
        stream.write(&request).map_err(|_| NodeWalkerError::Write)?;

        let mut response : Vec<u8> = Vec::new();
        let mut buffer = [0; 2048];
        let read = stream.read(&mut buffer).map_err(|_| NodeWalkerError::Read)?;
        let mut slice = buffer.get(0..read).unwrap().to_vec();
        response.append(&mut slice);

        stream.shutdown(Shutdown::Both).map_err(|_| NodeWalkerError::Shutdown)?;
        let ss = std::str::from_utf8(&response.as_slice()).unwrap();

        let mut result : Vec<String> = Vec::new();
        result.push(ss.to_string());
        Ok(result)
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