use rbtc::network::networkaddr::NetworkAddr;
use rbtc::network::version::Version;
use rbtc::network::verack::VerAck;
use rbtc::network::getaddr::GetAddr;
use rbtc::network::version::Service;
use rbtc::network::message::Payload;
use rbtc::network::message::{Message, Magic};
use rbtc::encode::encode::{Encodable, Decodable};
use rbtc::utils::hexdump;

use rand::Rng;

use std::net::{TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use std::fmt;
use std::io::{Cursor, Error, ErrorKind};
use std::sync::mpsc;
use std::thread;

 
#[derive(Debug)]
pub enum NodeWalkerError {
    Encode,
    Connect,
    ReadTimeout,
    Read,
    Write,
    WriteTimeout,
    Shutdown,
    DecodeMessage
}

impl fmt::Display for NodeWalkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct NodeWalker {
    stream: Option<TcpStream>,
    addr: SocketAddr,
}

impl NodeWalker {

    pub fn new(nodeip: &String) -> NodeWalker {

        let mut node_ip_port = nodeip.clone();
        node_ip_port.push_str(":8333");
        let addr : SocketAddr = node_ip_port.parse().unwrap();

        NodeWalker {
            stream: None,
            addr: addr,
        }
    }

    fn connect(&self) -> Result<TcpStream, NodeWalkerError> {

        let connect_timeout = std::time::Duration::from_secs(3);
        let read_timeout = std::time::Duration::from_secs(10);
        let write_timeout = std::time::Duration::from_secs(5);

        let stream = TcpStream::connect_timeout(&self.addr, connect_timeout).map_err(|_| NodeWalkerError::Connect)?;
        stream.set_read_timeout(Option::Some(read_timeout)).map_err(|_| NodeWalkerError::ReadTimeout)?;
        stream.set_write_timeout(Option::Some(write_timeout)).map_err(|_| NodeWalkerError::WriteTimeout)?;

        Ok(stream)
    }
    /// 
    /// https://en.itcoin.it/wiki/Version_Handshake
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
    pub fn walk(&self) -> Result<Vec<String>, NodeWalkerError> {

        let mut stream = self.connect()?;

        let version = self.version();
        let verack = self.send(&mut stream, version)?;

        println!("{:?}", verack);

        let verack_getaddr = self.verack_getaddr();
        let addr = self.send(&mut stream, verack_getaddr)?;

        let result = self.parse_addr(addr)?;
        Ok(result)
    }

    fn parse_addr(&self, rx: mpsc::Receiver<Vec<Message>>) -> Result<Vec<String>, NodeWalkerError> {

        let mut result : Vec<String> = Vec::new();
        while let Ok(messages) = rx.recv() {
            for message in messages {
                match message.payload {
                    Payload::Addr(addr) => {
                        for tna in addr.addrs {
                            let ip_port = format!("{}:{}", tna.addr.ip, tna.addr.port);
                            result.push(ip_port);
                        }
                    },
                    _ => {}
                };
            }
        }

        Ok(result)
    }

    fn send(&self, stream: &mut TcpStream, messages: Vec<Message>) -> Result<mpsc::Receiver<Vec<Message>>, NodeWalkerError> {

        let (tx, rx) = mpsc::channel();

        let mut request : Vec<u8> = Vec::new();
        messages.encode(&mut request).map_err(|_| NodeWalkerError::Encode)?;

        let hexcontent = hexdump::encode(request.clone());
        println!("{}", hexcontent);


        stream.write(&request).map_err(|_| NodeWalkerError::Write)?;

        let tx = tx.clone();
        let receiver_thread = thread::spawn(move || {

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
                if read < buffer.len() {
                    
                    let hexcontent2 = hexdump::encode(response.clone());
                    println!("{}", hexcontent2);

                    let mut cursor = Cursor::new(&response);
                    if let Ok(result) = <Vec<Message>>::decode(&mut cursor) {
                        response.clear();

                        if let Err(fail) = tx.send(result) {
                            break;
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    fn version(&self) -> Vec<Message> {

        let now = chrono::Local::now();
        let mut rng = rand::thread_rng();
        let nonce: u64 = rng.gen();

        let version = Version {
            version: 70002,
            services: Service::Network,
            timestamp: now.timestamp(),
            receiver: NetworkAddr {
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            sender: NetworkAddr {
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            nonce: nonce,
            user_agent: "/rbtc:0.17.0.1/".to_string(),
            start_height: 557409,
            relay: false,
        };

        let version = Payload::Version(version);
        vec![
            Message {
                magic: Magic::MainNet,
                payload: version
            }
        ]
    }

    fn verack_getaddr(&self) -> Vec<Message> {

        let now = chrono::Local::now();
        let mut rng = rand::thread_rng();
        let nonce: u64 = rng.gen();

        let verack = Payload::VerAck(VerAck {});
        let getaddr = Payload::GetAddr(GetAddr {});
        
        vec![
            Message {
                magic: Magic::MainNet,
                payload: verack
            },
            Message {
                magic: Magic::MainNet,
                payload: getaddr
            },
        ]
    }

}

#[cfg(test)]
mod test {

    use crate::walker::NodeWalker;

    #[test]
    fn when_walker_walk_localhost_then_return_some_strings() {

        let node = String::from("127.0.0.1");
        let walker = NodeWalker::new(&node);
        let result = walker.walk();

        assert!(result.is_ok());

        for line in result.unwrap() {
            println!("{}", line);
        }
    }

    #[test]
    fn when_walker_walk_workinghost_then_return_some_strings() {

        let node = String::from("142.93.2.255");
        let walker = NodeWalker::new(&node);
        let result = walker.walk();

        assert!(result.is_ok());

        for line in result.unwrap() {
            println!("{}", line);
        }
    }
}