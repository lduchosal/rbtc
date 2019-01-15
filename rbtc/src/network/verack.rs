use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#verack
/// 
/// # verack
/// 
/// The verack message is sent in reply to version. This message consists of only 
/// a message header with the command string "verack".
///
/// ## Hexdump of the verack message:
/// ```
/// 0000   F9 BE B4 D9 76 65 72 61  63 6B 00 00 00 00 00 00   ....verack......
/// 0010   00 00 00 00 5D F6 E0 E2                            ........
/// ```
/// 
/// ## Message header:
/// 
/// ```
///  F9 BE B4 D9                          - Main network magic bytes
///  76 65 72 61  63 6B 00 00 00 00 00 00 - "verack" command
///  00 00 00 00                          - Payload is 0 bytes long
///  5D F6 E0 E2                          - Checksum (little endian)
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct VerAck {
}

impl Encodable for VerAck {

    fn encode(&self, _: &mut Vec<u8>) -> Result<(), Error> {
        trace!("encode");
        Ok(())
    }
}

impl Decodable for VerAck {

    fn decode(_: &mut Cursor<&Vec<u8>>) -> Result<VerAck, Error> {
        trace!("decode");
        Ok(VerAck {})
    }
}

#[cfg(test)]
mod test {

    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::verack::VerAck;

    use std::io::{Read, Write, Cursor};
    use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn when_encode_verack_then_nothing_to_encode() {

        let message = VerAck {};
        let mut data : Vec<u8> = Vec::new();

        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(0, data.len())
    }

    #[test]
    fn when_decode_verack_then_nothing_to_encode() {

        let data : Vec<u8> = Vec::new();
        let mut read = Cursor::new(&data);
        let result = VerAck::decode(&mut read);

        let expected = VerAck {};

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

}