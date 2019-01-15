use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#ping
/// 
/// ping
/// The ping message is sent primarily to confirm that the TCP/IP connection is still valid. 
/// An error in transmission is presumed to be a closed connection and the address is 
/// removed as a current peer.
/// 
/// Payload:
/// ```
/// +------------+--------------+-----------+--------------+
/// | Field Size |  Description | Data type | Comments     |
/// +------------+--------------+-----------+--------------+
/// | 8          |  nonce       | uint64_t  | random nonce |
/// +------------+--------------+-----------+--------------+
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct Ping {
    nonce: u64
}

impl Encodable for Ping {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        trace!("encode");
        self.nonce.encode(w).map_err(|_| Error::PingNonce)?;
        Ok(())
    }
}

impl Decodable for Ping {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Ping, Error> {
        trace!("decode");
        let nonce = u64::decode(r).map_err(|_| Error::PingNonce)?;
        let result = Ping {
            nonce: nonce
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::ping::Ping;

    use std::io::{Read, Write, Cursor};
    use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn when_encode_ping_then_nothing_to_encode() {

        let message = Ping {
            nonce: 0
        };
        let mut data : Vec<u8> = Vec::new();

        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(8, data.len())
    }

    #[test]
    fn when_decode_ping_then_nothing_to_encode() {

        let data : Vec<u8> = vec![0u8; 8];
        let mut read = Cursor::new(&data);
        let result = Ping::decode(&mut read);

        let expected = Ping {
            nonce: 0
        };

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn when_decode_ping_one_then_nothing_to_encode() {

        let data : Vec<u8> = vec![1u8; 8];
        let mut read = Cursor::new(&data);
        let result = Ping::decode(&mut read);

        let expected = Ping {
            nonce: 72340172838076673
        };

        assert!(result.is_ok());

        let success = result.unwrap();
        assert_eq!(expected, success);
    }

}