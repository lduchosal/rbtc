use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#getaddr
/// 
/// getaddr
/// 
/// The getaddr message sends a request to a node asking for information 
/// about known active peers to help with finding potential nodes in the network. 
/// The response to receiving this message is to transmit one or more addr messages 
/// with one or more peers from a database of known active peers. 
/// 
/// The typical presumption is that a node is likely to be active if it has been sending 
/// a message within the last three hours.
/// 
/// No additional data is transmitted with this message.
/// 
#[derive(Debug, PartialEq)]
pub struct GetAddr {
}

impl Encodable for GetAddr {

    fn encode(&self, _: &mut Vec<u8>) -> Result<(), Error> {
        trace!("encode");
        Ok(())
    }
}

impl Decodable for GetAddr {

    fn decode(_: &mut Cursor<&Vec<u8>>) -> Result<GetAddr, Error> {
        trace!("decode");
        Ok(GetAddr {})
    }
}

#[cfg(test)]
mod test {

    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::getaddr::GetAddr;

    use std::io::{Read, Write, Cursor};
    use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn when_encode_getaddr_then_nothing_to_encode() {

        let message = GetAddr {};
        let mut data : Vec<u8> = Vec::new();

        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(0, data.len())
    }

    #[test]
    fn when_decode_getaddr_then_nothing_to_encode() {

        let data : Vec<u8> = Vec::new();
        let mut read = Cursor::new(&data);
        let result = GetAddr::decode(&mut read);

        let expected = GetAddr {};

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

}