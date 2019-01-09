use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::encode::varint::VarInt;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct Sha256 {
    pub hash: [u8; 32],
}

impl Encodable for Sha256 {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.write_all(&self.hash).map_err(|_| Error::WriteSha256)?;
        Ok(())
    }
}

impl Decodable for Sha256 {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Sha256, Error> {
        let hash = <[u8; 32]>::decode(r).map_err(|_| Error::ReadSha256)?;
        let sha = Sha256 {
            hash: hash
        };
        Ok(sha)
    }
}

impl Decodable for Vec<Sha256> {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Sha256>, Error> {
        
        let mut result : Vec<Sha256> = Vec::new();
        let count = VarInt::decode(r).map_err(|_| Error::Sha256Count)?;

        for _ in 0..count.0 {
            let locator = Sha256::decode(r)?;
            result.push(locator);
        }
        
        Ok(result)
    }
}

impl Encodable for Vec<Sha256> {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
    
        let varint = VarInt::new(self.len() as u64);
        varint.encode(w).map_err(|_| Error::Sha256Count)?;
        for sha in self {
            sha.encode(w)?;
        };

        Ok(())
    }
}