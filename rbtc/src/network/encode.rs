use crate::network::error::Error;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

pub trait Encodable {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error>;
}

pub trait Decodable : Sized {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Self, Error>;
}

impl Encodable for i64 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_i64::<LittleEndian>(*self).map_err(|_| Error::WriteI64)?;
        Ok(())
    }
}