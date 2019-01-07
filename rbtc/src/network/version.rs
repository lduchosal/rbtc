use crate::network::message::{NetworkMessage};
use crate::network::encode::{Encodable, Decodable};
use crate::network::networkaddr::NetworkAddr;
use crate::network::error::Error;
use crate::network::message::Command;

use std::io::{Write, Read, Cursor};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

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
/// +-------+----------------------+-----------------------------------------------------------------+
/// | Value | Name                 | Description                                                     |
/// +-------+----------------------+-----------------------------------------------------------------+
/// |    1  | NODE_NETWORK         | This node can be asked for full blocks instead of just headers. |
/// |    2  | NODE_GETUTXO         | See BIP 0064                                                    |
/// |    4  | NODE_BLOOM           | See BIP 0111                                                    | 
/// |    8  | NODE_WITNESS         | See BIP 0144                                                    | 
/// | 1024  | NODE_NETWORK_LIMITED | See BIP 0159                                                    |
/// +-------+----------------------+-----------------------------------------------------------------+
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum Service {
    Network = 1,
    GetUtxo = 2,
    Bloom = 4,
    Witness = 8,
    NetworkLimited = 1024,
}

impl Service {
    pub fn from_value(value :u64) -> Result<Service, Error> {
        match value {
            1 => Ok(Service::Network),
            2 => Ok(Service::GetUtxo),
            4 => Ok(Service::Bloom),
            8 => Ok(Service::Witness),
            1024 => Ok(Service::NetworkLimited),
            _ => Err(Error::ServiceMatch)
        }
    }
}

impl Encodable for Service {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        let clone = self.clone() as u64;
        clone.encode(w).map_err(|_| Error::Service)?;
        Ok(())
    }
}

impl Decodable for Service {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Service, Error> {
        let value = u64::decode(r).map_err(|_| Error::Service)?;
        Service::from_value(value)
    }
}

impl NetworkMessage for Version {

    fn command(&self) -> Command {
        Command::Version
    }
}

impl Encodable for Version {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        self.version.encode(w).map_err(|_| Error::VersionVersion)?;
        self.services.encode(w).map_err(|_| Error::VersionServices)?;
        self.timestamp.encode(w).map_err(|_| Error::VersionTimestamp)?;
        self.receiver.encode(w).map_err(|_| Error::VersionReceiver)?;
        self.sender.encode(w).map_err(|_| Error::VersionSender)?;
        self.nonce.encode(w).map_err(|_| Error::VersionNonce)?;

        let user_agent_bytes = self.user_agent.as_bytes();
        let user_agent_len = user_agent_bytes.len() as u8;

        user_agent_len.encode(w).map_err(|_| Error::VersionUserAgentLen)?;

        w.write_all(user_agent_bytes).map_err(|_| Error::VersionUserAgent)?;

        self.start_height.encode(w).map_err(|_| Error::VersionStartHeight)?;
        self.relay.encode(w).map_err(|_| Error::VersionRelay)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::network::version::Version;
    use crate::network::version::Service;
    use crate::network::networkaddr::NetworkAddr;
    use crate::network::message::NetworkMessage;
    use crate::network::encode::{Encodable, Decodable};
    use crate::utils::hexdump;

    use std::net::IpAddr;

     #[test]
    fn version_message_test() {
        let dump = "
00000000   72 11 01 00 01 00 00 00  00 00 00 00 e6 e0 84 53   ver.service.time
00000000   00 00 00 00 01 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 ff ff  00 00 00 00 00 00 01 00   ................
00000000   00 00 00 00 00 00 fd 87  d8 7e eb 43 64 f2 2c f5   ................
00000000   4d ca 59 41 2d b7 20 8d  47 d9 20 cf fc e8 3e e8   .........nonce..
00000000   10 2f 53 61 74 6f 73 68  69 3a 30 2e 39 2e 39 39   .useragent......
00000000   2f 2c 9f 04 00 01                                  height.relay.   
";

        // This message is from a satoshi node, morning of May 27 2014
        let original : Vec<u8> = hexdump::decode(dump);
        let service = Service::Network;
        let ip_receiver = IpAddr::V4("0.0.0.0".parse().unwrap());

        let version = Version {
            version: 70002,
            services: Service::Network,
            timestamp: 1401217254,
            receiver: NetworkAddr {
                services: service.clone(),
                ip: ip_receiver,
                port: 0
            },
            sender: NetworkAddr {
                services: service,
                ip: ip_receiver,
                port: 0
            },
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