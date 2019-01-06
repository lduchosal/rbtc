use crate::network::version::Service;
use crate::network::message::Encodable;
use crate::network::error::EncodeError;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::{Write};
use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
/// 
/// Network address
/// When a network address is needed somewhere, this structure is used. 
/// Network addresses are not prefixed with a timestamp in the version message.
///
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// | Field Size | Description | Data type | Comments                                                       |
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// |     4      | time        | uint32    | the Time (version >= 31402). Not present in version message.   |
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// |     8      | services    | uint64_t  | same service(s) listed in version                              |
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// |    16      | IPv6/4      | char[16]  | IPv6 address. Network byte order. The original client only     |
/// |            |             |           | supported IPv4 and only read the last 4 bytes to get the IPv4  |
/// |            |             |           | address. However, the IPv4 address is written into the message |
/// |            |             |           | as a 16 byte IPv4-mapped IPv6 address                          |
/// |            |             |           |                                                                |
/// |            |             |           | (12 bytes 00 00 00 00 00 00 00 00 00 00 FF FF, followed by the |
/// |            |             |           | 4 bytes of the IPv4 address).                                  |
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// |      2     | port        | uint16_t  | port number, network byte order                                |
/// +------------+-------------+-----------+----------------------------------------------------------------+
/// 
/// Hexdump example of Network address structure
///
/// 0000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
/// 0010   00 00 FF FF 0A 00 00 01  20 8D                    ........ .
///
/// Network address:
///  01 00 00 00 00 00 00 00                         - 1 (NODE_NETWORK: see services listed under version command)
///  00 00 00 00 00 00 00 00 00 00 FF FF 0A 00 00 01 - IPv6: ::ffff:a00:1 or IPv4: 10.0.0.1
///  20 8D                                           - Port 8333
/// 
#[derive(Debug)]
pub struct NetworkAddr {
    pub time: Option<u32>,
    pub services: Service,
    pub ip: IpAddr,
    pub port: u16
}

impl Encodable for NetworkAddr {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {

        if let Some(time) = self.time {
            w.write_u32::<LittleEndian>(time).map_err(|_| EncodeError::NetworkAddrTime)?;
        }
        self.services.encode(w).map_err(|_| EncodeError::NetworkAddrServices)?;
        self.ip.encode(w).map_err(|_| EncodeError::NetworkAddrIp)?;
        w.write_u16::<BigEndian>(self.port).map_err(|_| EncodeError::NetworkAddrPort)?;

        Ok(())
    }
}

impl Encodable for IpAddr {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {

        let ipv6 :Ipv6Addr = match *self {
            IpAddr::V4(ip4) => ip4.to_ipv6_mapped(),
            IpAddr::V6(ip6) => ip6,
        };
        w.write_all(&ipv6.octets()).map_err(|_| EncodeError::IpAddr)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::network::error::EncodeError;
    use crate::network::message::Encodable;
    use crate::network::networkaddr::NetworkAddr;
    use crate::network::version::Service;
    use crate::utils::hexdump;

    use std::net::{IpAddr};

    #[test]
    fn when_encode_10_0_0_1_then_same_as_hexdump() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 FF FF 0A 00 00 01  20 8D                     ..........
";

        // This message is from a satoshi node, morning of May 27 2014
        let original : Vec<u8> = hexdump::decode(dump);
        let ip = IpAddr::V4("10.0.0.1".parse().unwrap());
        let port = 8333;
        let service = Service::Network;

        let encodable = NetworkAddr {
            time: None,
            services: service,
            ip: ip,
            port: port
        };

        let mut data : Vec<u8> = Vec::new();
        let result = encodable.encode(&mut data);

        assert!(result.is_ok());
        assert_eq!(original, data);
    }


    #[test]
    fn when_encode_addr_0_0_0_0_port_0_then_same_as_hexdump() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000010   00 00 FF FF 00 00 00 00  00 00                     ..........
";

        let original : Vec<u8> = hexdump::decode(dump);
        let ip = IpAddr::V4("0.0.0.0".parse().unwrap());
        let port = 0;
        let service = Service::Network;

        let encodable = NetworkAddr {
            time: None,
            services: service,
            ip: ip,
            port: port
        };

        let mut data : Vec<u8> = Vec::new();
        let result = encodable.encode(&mut data);

        assert!(result.is_ok());
        assert_eq!(original, data);
    }


    #[test]
    fn when_encode_addr_ipv6_port_8333_then_same_as_hexdump() {

        let dump = "
00000000   01 00 00 00 00 00 00 00  FD 87 D8 7E EB 43 64 F2   ................
00000010   2C F5 4D CA 59 41 2D B7  20 8D                     ..........
";

        let original : Vec<u8> = hexdump::decode(dump);
        let ip = IpAddr::V6("FD87:D87E:EB43:64F2:2CF5:4DCA:5941:2DB7".parse().unwrap());
        let port = 8333;
        let service = Service::Network;

        let encodable = NetworkAddr {
            time: None,
            services: service,
            ip: ip,
            port: port
        };

        let mut data : Vec<u8> = Vec::new();
        let result = encodable.encode(&mut data);

        assert!(result.is_ok());
        assert_eq!(original, data);
    }

}