use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};



/// https://en.bitcoin.it/wiki/Protocol_documentation#Transaction_Verification
/// 
/// Variable length integer
/// Integer can be encoded depending on the represented value to save space. 
/// Variable length integers always precede an array/vector of a type of data 
/// that may vary in length. Longer numbers are encoded in little endian.
/// 
/// +----------------+-----------------+----------------------------------------------+
/// | Value          | Storage length  |  Format                                      |
/// +----------------+-----------------+----------------------------------------------+
/// | < 0xFD         | 1               |  uint8_t                                     |
/// +----------------+-----------------+----------------------------------------------+
/// | <= 0xFFFF      | 3               |  0xFD followed by the length as uint16_t     |
/// +----------------+-----------------+----------------------------------------------+
/// | <= 0xFFFF FFFF | 5               |  0xFE followed by the length as uint32_t     |
/// +----------------+-----------------+----------------------------------------------+
/// | -              | 9               |  0xFF followed by the length as uint64_t     |
/// +----------------+-----------------+----------------------------------------------+
/// 
/// If you're reading the Satoshi client code (BitcoinQT) it refers to this 
/// encoding as a "CompactSize". Modern BitcoinQT also has the CVarInt class 
/// which implements an even more compact integer for the purpose of local 
/// storage (which is incompatible with "CompactSize" described here). 
/// CVarInt is not a part of the protocol.
/// 

pub struct VarInt(pub u64);
impl VarInt {
    pub fn new(v :u64) -> VarInt {
        VarInt {
            0: v
        }
    }
}

impl Decodable for VarInt {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<VarInt, Error> {

        let varlen = r.read_u8().map_err(|_| Error::VarInt)?;
        match varlen {
            0xFD => u16::decode(r).map(|v| VarInt::new(v as u64)).map_err(|_| Error::VarIntFD),
            0xFE => u32::decode(r).map(|v| VarInt::new(v as u64)).map_err(|_| Error::VarIntFE),
            0xFF => u64::decode(r).map(|v| VarInt::new(v as u64)).map_err(|_| Error::VarIntFF),
            _ => Ok(VarInt::new(varlen as u64))
        }
    }
}
impl Encodable for VarInt {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        let size_enc : u8 = match self.0 {
            0...0xFC => self.0 as u8,
            0xFD...0xFFFF => 0xFD,
            0x10000...0xFFFFFFFF => 0xFE,
            _ => 0xFF,
        };

        size_enc.encode(w).map_err(|_| Error::VarInt)?;

        match self.0 {
            0...0xFC => {},
            0xFD...0xFFFF => {
                let s = self.0 as u16;
                s.encode(w).map_err(|_| Error::VarIntFD)?;
            },
            0x10000...0xFFFFFFFF => {
                let s = self.0 as u32;
                s.encode(w).map_err(|_| Error::VarIntFE)?;
            },
            _ => {
                self.0.encode(w).map_err(|_| Error::VarIntFF)?;
            },
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::encode::varint::VarInt;
    use crate::encode::error::Error;
    use crate::encode::encode::{Encodable, Decodable};
    
    use std::io::{Read, Write, Cursor};
    use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn when_decode_varint_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result.0, 0x00);
    }

    #[test]
    fn when_decode_varint_0xfc_then_1_byte() {

        let data : Vec<u8> = vec![0xfc, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result.0, 0xfc);
    }

    #[test]
    fn when_decode_varint_0xfd_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result.0, 0x00fe);
    }

    #[test]
    fn when_decode_varint_0xfd_fe_01_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result.0, 0x01fe);
    }


    #[test]
    fn when_decode_varint_0xfe_then_5_byte() {

        let data : Vec<u8> = vec![0xfe, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 5);

        let result = varint.unwrap();
        assert_eq!(result.0, 0x00010203);
    }
    #[test]
    fn when_decode_varint_0xff_then_9_byte() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 9);

        let result = varint.unwrap();
        assert_eq!(result.0, 0x0001020304050607);
    }


    #[test]
    fn when_decode_varint_0xff_too_small_then_fail_parseerror_varint_ff() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, Error::VarIntFF);
        } else {
            panic!("should have failed");
        }
    }


    #[test]
    fn when_decode_varint_0xfe_too_small_then_fail_parseerror_varint_fe() {

        let data : Vec<u8> = vec![0xfe, 0x07, 0x06, 0x05 ];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, Error::VarIntFE);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_decode_varint_0xfd_too_small_then_fail_parseerror_varint_fd() {

        let data : Vec<u8> = vec![0xfd, 0x07 ];
        let mut c = Cursor::new(data.as_ref());
        let varint = VarInt::decode(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, Error::VarIntFD);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_encode_varint_0xffffffffffffffff_then_size_9() {

        let data = VarInt::new(0xFFFFFFFFFFFFFFFF);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF) )
    }

    #[test]
    fn when_encode_varint_0xffffff_then_size_5() {

        let data = VarInt::new(0xFFFFFFFF);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFE, 0xFF, 0xFF, 0xFF, 0xFF) )
    }

    #[test]
    fn when_encode_varint_0xffff_then_size_3() {

        let data = VarInt::new(0xFFFF);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFD, 0xFF, 0xFF) )
    }
    #[test]
    fn when_encode_varint_0xfd_then_size_3() {

        let data = VarInt::new(0xFD);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFD, 0xFD, 0x00));
    }

    #[test]
    fn when_encode_varint_0xf0_then_size_1() {

        let data = VarInt::new(0xF0);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xF0 ));
    }

    #[test]
    fn when_encode_varint_0xfc_then_size_1() {

        let data = VarInt::new(0xFC);
        let mut result : Vec<u8> = Vec::new();
        let varint = data.encode(&mut result);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFC));
    }
}