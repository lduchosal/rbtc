use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::encode::varint::VarInt;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct Witness {
    pub data: Vec<u8>
}

impl Decodable for Vec<Witness> {
        
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Witness>, Error> {

        let mut result: Vec<Witness> = Vec::new();
        let count = VarInt::decode(r).map_err(|_| Error::WitnessesCount)?;
        for _ in 0..count.0 {
            let witness = Witness::decode(r)?;
            result.push(witness);
        }

        Ok(result)
    }
}

impl Decodable for Witness {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Witness, Error> {

        let varlen = VarInt::decode(r).map_err(|_| Error::WitnessLen)?;
        let mut data = vec![0u8; varlen.0 as usize];
        let mut data_ref = data.as_mut_slice();
        r.read_exact(&mut data_ref)
            .map_err(|_| Error::WitnessData)?;

        let result = Witness { data: data };

        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::encode::error::Error;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::block::witness;
    use crate::block::witness::Witness;

    use std::io::Cursor;

    #[test]
    fn decode_0x00_then_1_byte() {
        let data: Vec<u8> = vec![0x00];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_ok());
        assert_eq!(c.position(), 1);

        let result: Witness = result.unwrap();
        assert_eq!(result.data.len(), 0x00);
    }

    #[test]
    fn decode_0x01_then_1_byte() {
        let data: Vec<u8> = vec![0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_ok());
        assert_eq!(c.position(), 2);

        let result: Witness = result.unwrap();
        assert_eq!(result.data.len(), 0x01);
    }

    #[test]
    fn decode_0x02_then_2_byte() {
        let data: Vec<u8> = vec![0x02, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_ok());
        assert_eq!(c.position(), 3);

        let result: Witness = result.unwrap();
        assert_eq!(result.data.len(), 0x02);
    }

    #[test]
    fn decode_0x10_then_10_byte() {
        let data: Vec<u8> = vec![
            0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_ok());
        assert_eq!(c.position(), 0x11);

        let result: Witness = result.unwrap();
        assert_eq!(result.data.len(), 0x10);
    }

    #[test]
    fn decode_invalid_size_then_fail() {
        let data: Vec<u8> = vec![0x01];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_err());
        assert_eq!(c.position(), 0x01);

        if let Err(e) = result {
            assert_eq!(e, Error::WitnessData);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn decode_invalid_content_then_fail() {
        let data: Vec<u8> = vec![];
        let mut c = Cursor::new(data.as_ref());
        let result = Witness::decode(&mut c);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e, Error::WitnessLen);
        } else {
            panic!("should have failed");
        }
    }
}
