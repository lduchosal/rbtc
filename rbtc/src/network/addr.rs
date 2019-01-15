use crate::encode::error::Error;
use crate::encode::varint::VarInt;
use crate::encode::encode::{Encodable, Decodable};
use crate::network::networkaddr::TimedNetworkAddr;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#addr
/// 
/// # addr
/// Provide information on known nodes of the network. Non-advertised nodes 
/// should be forgotten after typically 3 hours
/// 
/// ## Payload:
/// ```
/// Len  | Description | Data type | Comments
/// 1+   | count       | var_int   | Number of address entries (max: 1000)
/// 30x? | addr_list   | u32       | Address of other nodes on the network. 
///      |             | + IpAddr  | version < 209 will only read the first one. 
///      |             | ]         | The u32 is a timestamp (see note below).
/// ```
/// 
/// Note: Starting version 31402, addresses are prefixed with a timestamp. 
/// If no timestamp is present, the addresses should not be relayed to 
/// other peers, unless it is indeed confirmed they are up.
/// 
/// ## Hexdump example of addr message:
/// ```
/// 0000   F9 BE B4 D9 61 64 64 72  00 00 00 00 00 00 00 00   ....addr........
/// 0010   1F 00 00 00 ED 52 39 9B  01 E2 15 10 4D 01 00 00   .....R9.....M...
/// 0020   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 FF   ................
/// 0030   FF 0A 00 00 01 20 8D                               ..... .
/// ```
/// 
/// ## Message Header:
/// 
/// ```
///  F9 BE B4 D9                                     - Main network magic bytes
///  61 64 64 72  00 00 00 00 00 00 00 00            - "addr"
///  1F 00 00 00                                     - payload is 31 bytes long
///  ED 52 39 9B                                     - payload checksum (little endian)
/// ```
/// 
/// Payload:
/// ```
///  01                                              - 1 address in this message
/// ```
/// 
/// Address:
/// ```
///  E2 15 10 4D                                     - Mon Dec 20 21:50:10 EST 2010 (only when version is >= 31402)
///  01 00 00 00 00 00 00 00                         - 1 (NODE_NETWORK service - see version message)
///  00 00 00 00 00 00 00 00 00 00 FF FF 0A 00 00 01 - IPv4: 10.0.0.1, IPv6: ::ffff:10.0.0.1 (IPv4-mapped IPv6 address)
///  20 8D                                           - port 8333
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct Addr {
    pub addrs: Vec<TimedNetworkAddr>,
}

impl Encodable for Addr {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        trace!("encode");
        self.addrs.encode(w)?;
        Ok(())
    }
}

impl Decodable for Addr {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Addr, Error> {
        trace!("decode");
        let addrs = <Vec<TimedNetworkAddr>>::decode(r)?;
        let result= Addr {
            addrs: addrs
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::addr::Addr;
    use crate::network::networkaddr::TimedNetworkAddr;
    use crate::network::networkaddr::NetworkAddr;

    use std::io::{Read, Write, Cursor};
    use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn when_encode_addr_then_nothing_to_encode() {

        let addrs : Vec<TimedNetworkAddr> = Vec::new();
        let message = Addr {
            addrs: addrs
        };        
        
        let mut data : Vec<u8> = Vec::new();
        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(1, data.len())
    }

    #[test]
    fn when_decode_addr_then_nothing_to_encode() {

        let data : Vec<u8> = vec![ 0 ];
        let mut read = Cursor::new(&data);
        let result = Addr::decode(&mut read);

        let addrs : Vec<TimedNetworkAddr> = Vec::new();
        let expected = Addr {
            addrs: addrs
        };

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

}