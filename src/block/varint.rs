
use crate::block::error::EncodeError;

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
pub(crate) fn parse_varint(r: &mut Cursor<&Vec<u8>>) -> Result<usize, EncodeError> {

    let varlen = r.read_u8().map_err(|_| EncodeError::VarInt)?;
    match varlen {
        0xFD => r.read_u16::<LittleEndian>().map(|v| v as usize).map_err(|_| EncodeError::VarIntFD),
        0xFE => r.read_u32::<LittleEndian>().map(|v| v as usize).map_err(|_| EncodeError::VarIntFE),
        0xFF => r.read_u64::<LittleEndian>().map(|v| v as usize).map_err(|_| EncodeError::VarIntFF),
        _ => Ok(varlen as usize)
    }

}

#[derive(PartialEq, Debug)]
pub enum EncodeError {
    VarIntData,
    VarIntFD,
    VarIntFDdata,
    VarIntFE,
    VarIntFEData,
    VarIntFF,
    VarIntFFData
}

pub(crate) fn encode(w: &mut Vec<u8>, size: u64) -> Result<(), EncodeError> {

    let size_enc : u8 = match size {
        0...0xFC => size as u8,
        0xFD...0xFFFF => 0xFD,
        0x10000...0xFFFFFFFF => 0xFE,
        _ => 0xFF,
    };

    w.write_u8(size_enc).map_err(|_| EncodeError::VarIntData)?;

    match size {
        0...0xFC => {},
        0xFD...0xFFFF => {
            w.write_u16::<LittleEndian>(size as u16).map_err(|_| EncodeError::VarIntFDdata)?;
        },
        0x10000...0xFFFFFFFF => {
            w.write_u32::<LittleEndian>(size as u32).map_err(|_| EncodeError::VarIntFEData)?;
        },
        _ => {
            w.write_u64::<LittleEndian>(size).map_err(|_| EncodeError::VarIntFFData)?;
        },
    };

    Ok(())

}

#[cfg(test)]
mod test {

    use crate::block::varint;
    use crate::block::error::EncodeError;
    use std::io::Cursor;

    #[test]
    fn when_parse_varint_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result, 0x00);
    }

    #[test]
    fn when_parse_varint_0xfc_then_1_byte() {

        let data : Vec<u8> = vec![0xfc, 0x00, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 1);

        let result = varint.unwrap();
        assert_eq!(result, 0xfc);
    }

    #[test]
    fn when_parse_varint_0xfd_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result, 0x00fe);
    }

    #[test]
    fn when_parse_varint_0xfd_fe_01_then_3_byte() {

        let data : Vec<u8> = vec![0xfd, 0xfe, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 3);

        let result = varint.unwrap();
        assert_eq!(result, 0x01fe);
    }


    #[test]
    fn when_parse_varint_0xfe_then_5_byte() {

        let data : Vec<u8> = vec![0xfe, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 5);

        let result = varint.unwrap();
        assert_eq!(result, 0x00010203);
    }
    #[test]
    fn when_parse_varint_0xff_then_9_byte() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_ok());
        assert_eq!(c.position(), 9);

        let result = varint.unwrap();
        assert_eq!(result, 0x0001020304050607);
    }


    #[test]
    fn when_parse_varint_0xff_too_small_then_fail_parseerror_varint_ff() {

        let data : Vec<u8> = vec![0xff, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, EncodeError::VarIntFF);
        } else {
            panic!("should have failed");
        }
    }


    #[test]
    fn when_parse_varint_0xfe_too_small_then_fail_parseerror_varint_fe() {

        let data : Vec<u8> = vec![0xfe, 0x07, 0x06, 0x05 ];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, EncodeError::VarIntFE);
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn when_parse_varint_0xfd_too_small_then_fail_parseerror_varint_fd() {

        let data : Vec<u8> = vec![0xfd, 0x07 ];
        let mut c = Cursor::new(data.as_ref());
        let varint = varint::parse_varint(&mut c);
        assert!(varint.is_err());
        assert_eq!(c.position(), 1);

        if let Err(e) = varint {
            assert_eq!(e, EncodeError::VarIntFD);
        } else {
            panic!("should have failed");
        }
    }
}