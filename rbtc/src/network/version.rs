use crate::network::message::{NetworkMessage, Encodable};
use crate::network::networkaddr::NetworkAddr;
use crate::network::error::EncodeError;
use crate::network::message::Command;

use std::io::{Write};
use byteorder::{LittleEndian, WriteBytesExt};

/// The `version` message
/// https://en.bitcoin.it/wiki/Protocol_documentation#version
/// 
/// When a node creates an outgoing connection, it will immediately advertise its version. 
/// The remote node will respond with its version. No further communication is possible until 
/// both peers have exchanged their version.
///  
/// Payload:
/// 
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// | Field Size | Description  | Data type | Comments                                                     |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// |      4     | version      | int32_t   | Identifies protocol version being used by the node           |
/// |      8     | services     | uint64_t  | bitfield of features to be enabled for this connection       |
/// |      8     | timestamp    | int64_t   | standard UNIX timestamp in seconds                           |
/// |     26     | addr_recv    | net_addr  | The network address of the node receiving this message       |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// | Fields below require version ≥ 106                                                                   |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// |     26     | addr_from    | net_addr  | The network address of the node emitting this message        |
/// |      8     | nonce        | uint64_t  | Node random nonce, randomly generated every time a version   |
/// |            |              |           | packet is sent. This nonce is used to detect connections to  |
/// |            |              |           | self.                                                        |
/// |      ?     | user_agent   | var_str   | User Agent (0x00 if string is 0 bytes long)                  |
/// |      4     | start_height | int32_t   | The last block received by the emitting node                 |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// | Fields below require version ≥ 70001                                                                 |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// |      1     | relay        | bool      | Whether the remote peer should announce relayed transactions |
/// |            |              |           | or not, see BIP 0037                                         |
/// +------------+--------------+-----------+--------------------------------------------------------------+
/// 
/// A "verack" packet shall be sent if the version packet was accepted.
#[derive(Debug)]
pub struct Version {
    pub version: i32,
    pub services: Service,
    pub timestamp: i64,
    pub receiver: NetworkAddr,
    pub sender: NetworkAddr,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}

/// The following services are currently assigned:
/// https://en.bitcoin.it/wiki/Protocol_documentation#version
/// 
/// ```
/// +-------+----------------------+-----------------------------------------------------------------+
/// | Value | Name                 | Description                                                     |
/// +-------+----------------------+-----------------------------------------------------------------+
/// |    1  | NODE_NETWORK         | This node can be asked for full blocks instead of just headers. |
/// |    2  | NODE_GETUTXO         | See BIP 0064                                                    |
/// |    4  | NODE_BLOOM           | See BIP 0111                                                    | 
/// |    8  | NODE_WITNESS         | See BIP 0144                                                    | 
/// | 1024  | NODE_NETWORK_LIMITED | See BIP 0159                                                    |
/// +-------+----------------------+-----------------------------------------------------------------+
/// ```
#[derive(Debug, Clone)]
pub enum Service {
    Network = 1,
    GetUtxo = 2,
    Bloom = 4,
    Witness = 8,
    NetworkLimited = 1024,
}

impl Encodable for Service {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {
        w.write_u64::<LittleEndian>(self.clone() as u64).map_err(|_| EncodeError::Service)?;
        Ok(())
    }
}

impl NetworkMessage for Version {

    fn command(&self) -> Command {
        Command::Version
    }
}

impl Encodable for Version {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {

        w.write_i32::<LittleEndian>(self.version).map_err(|_| EncodeError::VersionVersion)?;
        self.services.encode(w).map_err(|_| EncodeError::VersionServices)?;
        //w.write_u64::<LittleEndian>(self.services.clone() as u64).map_err(|_| EncodeError::VersionServices)?;
        w.write_i64::<LittleEndian>(self.timestamp).map_err(|_| EncodeError::VersionTimestamp)?;
        self.receiver.encode(w).map_err(|_| EncodeError::VersionReceiver)?;
        self.sender.encode(w).map_err(|_| EncodeError::VersionSender)?;
        w.write_u64::<LittleEndian>(self.nonce).map_err(|_| EncodeError::VersionNonce)?;

        let b_user_agent = self.user_agent.as_bytes();
        w.write_u8(b_user_agent.len() as u8).map_err(|_| EncodeError::VersionUserAgentLen)?;
        w.write_all(b_user_agent).map_err(|_| EncodeError::VersionUserAgent)?;
        w.write_i32::<LittleEndian>(self.start_height).map_err(|_| EncodeError::VersionStartHeight)?;
        w.write_u8(if self.relay { 1 } else { 0 }).map_err(|_| EncodeError::VersionRelay)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::network::version::Version;
    use crate::network::version::Service;
    use crate::network::networkaddr::NetworkAddr;
    use crate::network::message::Encodable;
    use crate::network::message::NetworkMessage;
    use crate::utils::hexdump;

    use std::net::IpAddr;

     #[test]
    fn version_message_test() {

        let dump = "
00000000   72 11 01 00 01 00 00 00  00 00 00 00 e6 e0 84 53   ................
00000000   00 00 00 00 01 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 ff ff  00 00 00 00 00 00 01 00   ................
00000000   00 00 00 00 00 00 fd 87  d8 7e eb 43 64 f2 2c f5   ................
00000000   4d ca 59 41 2d b7 20 8d  47 d9 20 cf fc e8 3e e8   .........nonce..
00000000   10 2f 53 61 74 6f 73 68  69 3a 30 2e 39 2e 39 39   .useragent......
00000000   2f 2c 9f 04 00 01                                  height.relay.   
";

        // This message is from a satoshi node, morning of May 27 2014
        let original : Vec<u8> = hexdump::parse(dump);

        let service = Service::Network;
        let ip_receiver = IpAddr::V4("0.0.0.0".parse().unwrap());
        let version = Version {
            version: 70002,
            services: service.clone(),
            timestamp: 0,
            receiver: NetworkAddr {
                time: None,
                services: service.clone(),
                ip: ip_receiver,
                port: 0
            },
            //[ 1, 0, 0, 0, 0, 0, 0, 0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ],
            sender: NetworkAddr {
                time: None,
                services: service,
                ip: ip_receiver,
                port: 0
            },
            // [ 1, 0, 0, 0, 0, 0, 0, 0, 0xfd, 0x87, 0xd8, 0x7e, 0xeb, 0x43, 0x64, 0xf2, 0x2c, 0xf5, 0x4d, 0xca, 0x59, 0x41, 0x2d, 0xb7, 0x20, 0x8d ],
            nonce: 0xE83EE8FCCF20D947,
            user_agent: "/Satoshi:0.9.99/".to_string(),
            start_height: 0x00049F2C,
            relay: true,
        };
        
        let mut result : Vec<u8> = Vec::new();
        let encoded = version.encode(&mut result);
        assert!(encoded.is_ok());

        assert_eq!(original, result);
    }
}