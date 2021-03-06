extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::utils::sha256::Sha256;
use crate::encode::encode::{Encodable, Decodable};
use crate::network::command::CommandString;
use crate::network::getheaders::GetHeaders;
use crate::network::message::{Payload, Message, Magic};

#[test]
fn test() {

    let mut locators : Vec<Sha256> = Vec::new();
    let loc1 = Sha256 {
        hash: [
            0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11, 
            0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13, 
            0x14, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x15, 
            0x16, 0x16, 0x16, 0x16, 0x00, 0x00, 0x00, 0x00
        ]
    };

    let loc2 = Sha256 {
        hash: [
            0x20, 0x20, 0x20, 0x20, 0x21, 0x21, 0x21, 0x21, 
            0x22, 0x22, 0x22, 0x22, 0x23, 0x23, 0x23, 0x23, 
            0x24, 0x24, 0x24, 0x24, 0x25, 0x25, 0x25, 0x25, 
            0x26, 0x26, 0x26, 0x26, 0x00, 0x00, 0x00, 0x00
        ]
    };
    locators.push(loc1);
    locators.push(loc2);

    let stop = Sha256 { hash: [0u8; 32] };

    let getheadermessage = GetHeaders {
        version: 70001,
        locators: locators,
        stop: stop
    };
    let message = Message {
        magic: Magic::MainNet,
        payload: Payload::GetHeaders(getheadermessage)
    };

    let mut data : Vec<u8> = Vec::new();
    let ok = message.encode(&mut data);
    assert!(ok.is_ok());
    
    let addr = "127.0.0.1:6142".parse().unwrap();
    
    let client  = TcpStream::connect(&addr).and_then(|stream| {
        io::write_all(stream, data).then(|_| {
            Ok(())
        })
    })
    .map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        //
        // In our example, we are only going to log the error to STDOUT.
        println!("connection error = {:?}", err);
        panic!("Should have suzceed");
    });

    tokio::run(client);
}