use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(PartialEq, Debug)]
pub struct CommandString(pub String);

#[derive(PartialEq, Debug)]
pub enum Command {
    Version,
    VerAck,
    GetHeaders,
    GetAddr,
    Alert,
    Addr,
    Ping,
    Pong,
    Inv,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        trace!("from_str");
         match s {
            "version" => Ok(Command::Version),
            "getheaders" => Ok(Command::GetHeaders),
            "getaddr" => Ok(Command::GetAddr),
            "verack" => Ok(Command::VerAck),
            "alert" => Ok(Command::Alert),
            "addr" => Ok(Command::Addr),
            "ping" => Ok(Command::Ping),
            "pong" => Ok(Command::Pong),
            "inv" => Ok(Command::Inv),
            _ => Err(())
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        trace!("to_string");
        match self {
            Command::Version => "version",
            Command::GetHeaders => "getheaders",
            Command::GetAddr => "getaddr",
            Command::VerAck => "verack",
            Command::Alert => "alert",
            Command::Addr => "addr",
            Command::Ping => "ping",
            Command::Pong => "pong",
            Command::Inv => "inv",
        }.to_owned()
    }
}

impl CommandString {
    pub fn to_command(&self) -> Result<Command, ()> {
        trace!("to_command");
        Command::from_str(self.0.as_ref())
    }
}

impl Encodable for CommandString {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        trace!("encode");
        let mut command = format!("{:}\0\0\0\0\0\0\0\0\0\0\0\0", self.0)
            .to_lowercase();
        command.truncate(12);
        w.write_all(command.as_bytes()).map_err(|_| Error::Command)?;

        Ok(())
    }
}

impl Decodable for CommandString {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<CommandString, Error> {
        
        trace!("decode");
        let buffer = <[u8; 12]>::decode(r).map_err(|_| Error::Command)?;
        let mut s = String::from_utf8(buffer.to_vec()).map_err(|_| Error::CommandDecode)?;
        s.retain(|c| c != (0 as char));

        let result = CommandString(s);
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::network::command::CommandString;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::utils::hexdump;

    use std::io::{Write, Read, Cursor};

     #[test]
    fn when_command_getaddr_then_sucess() {
        let dump = "
00000000   67 65 74 61 64 64 72 00  00 00 00 00               getaddr.......
";

        // This message is from a satoshi node, morning of May 27 2014
        let original : Vec<u8> = hexdump::decode(dump);
        let mut hex = Cursor::new(&original);

        let decode = CommandString::decode(&mut hex);
        assert!(decode.is_ok());

        let result = decode.unwrap();

        let expected = CommandString("getaddr".to_string());

        assert_eq!(expected, result);
    }


     #[test]
    fn when_command_version_then_sucess() {
        let dump = "
00000000   76 65 72 73 69 6f 6e 00  00 00 00 00               version......
";

        // This message is from a satoshi node, morning of May 27 2014
        let original : Vec<u8> = hexdump::decode(dump);
        let mut hex = Cursor::new(&original);

        let decode = CommandString::decode(&mut hex);
        assert!(decode.is_ok());

        let result = decode.unwrap();

        let expected = CommandString("version".to_string());

        assert_eq!(expected, result);
    }

}