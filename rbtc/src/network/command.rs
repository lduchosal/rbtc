use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

pub struct CommandString(pub String);

impl Encodable for CommandString {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        let mut command = format!("{:}\0\0\0\0\0\0\0\0\0\0\0\0", self.0)
            .to_lowercase();
        command.truncate(12);
        w.write_all(command.as_bytes()).map_err(|_| Error::Command)?;

        Ok(())
    }
}

impl Decodable for CommandString {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<CommandString, Error> {
        
        let mut buffer = [0u8; 12];
        r.read_exact(&mut buffer).map_err(|_| Error::Command)?;
        let s = String::from_utf8(buffer.to_vec()).map_err(|_| Error::CommandDecode)?;
        let result = CommandString(s);
        Ok(result)
    }
}
