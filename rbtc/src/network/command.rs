use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

pub struct CommandString(pub String);

#[derive(PartialEq, Debug)]
pub enum Command {
    Version,
    GetHeaders,
    GetAddr
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
         match s {
            "version" => Ok(Command::Version),
            "getheaders" => Ok(Command::GetHeaders),
            "getaddr" => Ok(Command::GetAddr),
            _ => Err(())
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Version => "version",
            Command::GetHeaders => "getheaders",
            Command::GetAddr => "getaddr",
        }.to_owned()
    }
}

impl CommandString {
    pub fn to_command(&self) -> Result<Command, ()> {
        Command::from_str(self.0.as_ref())
    }
}

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
