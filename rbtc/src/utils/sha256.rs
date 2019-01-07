use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

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
        let mut sha = Sha256 {
            hash: [0u8; 32]
        };
        r.read_exact(&mut sha.hash).map_err(|_| Error::ReadSha256)?;
        Ok(sha)
    }
}

