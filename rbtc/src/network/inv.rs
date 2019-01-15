use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::encode::varint::VarInt;


use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation#inv
/// 
/// # inv
/// 
/// Allows a node to advertise its knowledge of one or more objects. It can be received unsolicited, 
/// or in reply to getblocks.
/// 
/// Payload (maximum 50,000 entries, which is just over 1.8 megabytes):
/// 
/// ```
/// +------+-------------+------------+-----------------------------+
/// | Size | Description | Data type  | Comments                    |
/// +------+-------------+------------+-----------------------------+
/// |   1+ | count       | var_int    | Number of inventory entries |
/// | 36x? | inventory   | inv_vect[] | Inventory vectors           |
/// +------+-------------+------------+-----------------------------+
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct Inv {
    data: Vec<u8>,
}

impl Encodable for Inv {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        trace!("encode");
        let varint = VarInt::new(self.data.len() as u64);
        varint.encode(w)?;
        self.data.encode(w)?;
        Ok(())
    }
}

impl Decodable for Inv {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Inv, Error> {

        trace!("decode");
        let varlen = VarInt::decode(r).map_err(|_| Error::InvLen)?;
        let mut data = vec![0u8; varlen.0 as usize];
        let mut data_ref = data.as_mut_slice();
        r.read_exact(&mut data_ref).map_err(|_| Error::InvMessage)?;

        let result = Inv {
            data: data
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::encode::encode::{Encodable, Decodable};
    use crate::network::inv::Inv;

    use std::io::Cursor;

    #[test]
    fn when_encode_inv_then_nothing_to_encode() {

        let message = Inv {
            data: Vec::new()
        };
        let mut data : Vec<u8> = Vec::new();

        let result = message.encode(&mut data);
        assert!(result.is_ok());
        assert_eq!(1, data.len())
    }

    #[test]
    fn when_decode_inv_then_nothing_to_encode() {

        let data : Vec<u8> = vec![ 0x0 ];
        let mut read = Cursor::new(&data);
        let result = Inv::decode(&mut read);

        let expected = Inv {
            data: Vec::new()
        };

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

}