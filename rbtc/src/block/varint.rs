use crate::block::error::Error;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


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
pub(crate) fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<u64, Error> {

    let varlen = r.read_u8().map_err(|_| Error::VarInt)?;
    match varlen {
        0xFD => r.read_u16::<LittleEndian>().map(|v| v as u64).map_err(|_| Error::VarIntFD),
        0xFE => r.read_u32::<LittleEndian>().map(|v| v as u64).map_err(|_| Error::VarIntFE),
        0xFF => r.read_u64::<LittleEndian>().map(|v| v as u64).map_err(|_| Error::VarIntFF),
        _ => Ok(varlen as u64)
    }

}

pub(crate) fn encode(w: &mut Vec<u8>, size: u64) -> Result<(), Error> {

    let size_enc : u8 = match size {
        0...0xFC => size as u8,
        0xFD...0xFFFF => 0xFD,
        0x10000...0xFFFFFFFF => 0xFE,
        _ => 0xFF,
    };

    w.write_u8(size_enc).map_err(|_| Error::VarInt)?;

    match size {
        0...0xFC => {},
        0xFD...0xFFFF => {
            w.write_u16::<LittleEndian>(size as u16).map_err(|_| Error::VarIntFD)?;
        },
        0x10000...0xFFFFFFFF => {
            w.write_u32::<LittleEndian>(size as u32).map_err(|_| Error::VarIntFE)?;
        },
        _ => {
            w.write_u64::<LittleEndian>(size).map_err(|_| Error::VarIntFF)?;
        },
    };

    Ok(())
}

#[cfg(test)]
mod test {

    use crate::block::varint;
    use crate::block::varint::Error;
    use std::io::Cursor;

    #[test]
    fn when_decode_varint_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result, 0x00);
    }

    #[test]
    fn when_decode_varint_0xfc_then_1_byte() {

        let data : Vec<u8> = vec![0xfc, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result, 0xfc);
    }

    #[test]
    fn when_decode_varint_0xfd_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result, 0x00fe);
    }

    #[test]
    fn when_decode_varint_0xfd_fe_01_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result, 0x01fe);
    }


    #[test]
    fn when_decode_varint_0xfe_then_5_byte() {

        let data : Vec<u8> = vec![0xfe, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 5);

        let result = varint.unwrap();
        assert_eq!(result, 0x00010203);
    }
    #[test]
    fn when_decode_varint_0xff_then_9_byte() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 9);

        let result = varint.unwrap();
        assert_eq!(result, 0x0001020304050607);
    }


    #[test]
    fn when_decode_varint_0xff_too_small_then_fail_parseerror_varint_ff() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::decode(&mut c);
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
        let varint = varint::decode(&mut c);
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
        let varint = varint::decode(&mut c);
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

        let data : u64 = 0xFFFFFFFFFFFFFFFF;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF) )
    }

    #[test]
    fn when_encode_varint_0xffffff_then_size_5() {

        let data : u64 = 0xFFFFFFFF;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFE, 0xFF, 0xFF, 0xFF, 0xFF) )
    }

    #[test]
    fn when_encode_varint_0xffff_then_size_3() {

        let data : u64 = 0xFFFF;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFD, 0xFF, 0xFF) )
    }
    #[test]
    fn when_encode_varint_0xfd_then_size_3() {

        let data : u64 = 0xFD;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFD, 0xFD, 0x00));
    }

    #[test]
    fn when_encode_varint_0xf0_then_size_1() {

        let data : u64 = 0xF0;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xF0 ));
    }
    #[test]
    fn when_encode_varint_0xfc_then_size_1() {

        let data : u64 = 0xFC;
        let mut result : Vec<u8> = Vec::new();
        let varint = varint::encode(&mut result, data);

        assert!(varint.is_ok());
        assert_eq!(result, vec!(0xFC));
    }
}