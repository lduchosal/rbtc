use crate::encode::error::Error;
use crate::encode::varint::VarInt;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

pub trait Encodable {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error>;
}

pub trait NetworkEncodable {
    fn encode_network_byte_order(&self, w: &mut Vec<u8>) -> Result<(), Error>;
}

pub trait Decodable : Sized {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Self, Error>;
}

pub trait NetworkDecodable : Sized {
    fn decode_network_byte_order(r: &mut Cursor<&Vec<u8>>) -> Result<Self, Error>;
}

impl Encodable for i64 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_i64::<LittleEndian>(*self).map_err(|_| Error::WriteI64)?;
        Ok(())
    }
}

impl Decodable for i64 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<i64, Error> {
        let result = r.read_i64::<LittleEndian>().map_err(|_| Error::ReadI64)?;
        Ok(result)
    }
}

impl Encodable for i32 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_i32::<LittleEndian>(*self).map_err(|_| Error::WriteI32)?;
        Ok(())
    }
}

impl Decodable for i32 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<i32, Error> {
        let result = r.read_i32::<LittleEndian>().map_err(|_| Error::ReadI32)?;
        Ok(result)
    }
}

impl Encodable for i16 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_i16::<LittleEndian>(*self).map_err(|_| Error::WriteI16)?;
        Ok(())
    }
}

impl Decodable for i16 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<i16, Error> {
        let result = r.read_i16::<LittleEndian>().map_err(|_| Error::ReadI16)?;
        Ok(result)
    }
}

impl Encodable for i8 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_i8(*self).map_err(|_| Error::WriteI8)?;
        Ok(())
    }
}

impl Decodable for i8 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<i8, Error> {
        let result = r.read_i8().map_err(|_| Error::ReadI8)?;
        Ok(result)
    }
}

impl Encodable for u64 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_u64::<LittleEndian>(*self).map_err(|_| Error::WriteU64)?;
        Ok(())
    }
}

impl Decodable for u64 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<u64, Error> {
        let result = r.read_u64::<LittleEndian>().map_err(|_| Error::ReadU64)?;
        Ok(result)
    }
}

impl Encodable for u32 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_u32::<LittleEndian>(*self).map_err(|_| Error::WriteU32)?;
        Ok(())
    }
}

impl Decodable for u32 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<u32, Error> {
        let result = r.read_u32::<LittleEndian>().map_err(|_| Error::ReadU32)?;
        Ok(result)
    }
}

impl Encodable for u16 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_u16::<LittleEndian>(*self).map_err(|_| Error::WriteU16)?;
        Ok(())
    }
}

impl Decodable for u16 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<u16, Error> {
        let result = r.read_u16::<LittleEndian>().map_err(|_| Error::ReadU16)?;
        Ok(result)
    }
}

impl NetworkEncodable for u16 {
    fn encode_network_byte_order(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_u16::<BigEndian>(*self).map_err(|_| Error::WriteU16)?;
        Ok(())
    }
}

impl NetworkDecodable for u16 {

    fn decode_network_byte_order(r: &mut Cursor<&Vec<u8>>) -> Result<u16, Error> {
        let result = r.read_u16::<BigEndian>().map_err(|_| Error::ReadU16)?;
        Ok(result)
    }
}

impl Encodable for u8 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_u8(*self).map_err(|_| Error::WriteU8)?;
        Ok(())
    }
}

impl Decodable for u8 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<u8, Error> {
        let result = r.read_u8().map_err(|_| Error::ReadU8)?;
        Ok(result)
    }
}

impl Encodable for bool {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        let value : u8 = if *self { 1 } else { 0 };
        value.encode(w).map_err(|_| Error::WriteBool)?;
        Ok(())
    }
}

impl Decodable for bool {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<bool, Error> {
        let b = u8::decode(r).map_err(|_| Error::ReadBool)?;
        let result = match b {
            0 => false,
            _ => true
        };
        Ok(result)
    }
}

macro_rules! impl_array {
    ( $size:expr ) => (

        impl Encodable for [u8; $size] {
            fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
                w.write_all(self).map_err(|_| Error::WriteAll)?;
                Ok(())
            }
        }
        impl Decodable for [u8; $size]  {
            fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<[u8; $size], Error> {
                let mut result = [0u8; $size];
                r.read_exact(&mut result).map_err(|_| Error::ReadExact)?;
                Ok(result)
            }
        }
    );
}

impl_array!(2);
impl_array!(4);
impl_array!(8);
impl_array!(12);
impl_array!(16);
impl_array!(32);

impl Encodable for Vec<u8> {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_all(&self).map_err(|_| Error::VecContent)?;
        Ok(())
    }
}

impl Decodable for Vec<u8> {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<u8>, Error> {

        let varint = VarInt::decode(r).map_err(|_| Error::VecLen)?;
        let mut content = vec![0u8; varint.0 as usize];
        let mut content_ref = content.as_mut_slice();
        r.read_exact(&mut content_ref).map_err(|_| Error::VecContent)?;

        Ok(content)
    }
}
