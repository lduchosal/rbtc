use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::network::command::CommandString;

use crate::network::getheaders::GetHeaders;
use crate::network::getaddr::GetAddr;
use crate::network::version::Version;

use sha2::{Sha256, Digest};

use std::fmt;
use std::str::FromStr;
use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};


/// https://en.bitcoin.it/wiki/Protocol_documentation
/// 
/// Known magic values:
/// ```
/// +-----------+-------------+-------------------+
/// | Network   | Magic value | Sent over wire as |
/// +-----------+-------------+-------------------+
/// | main      | 0xD9B4BEF9  | F9 BE B4 D9       |
/// +-----------+-------------+-------------------+
/// | testnet   | 0xDAB5BFFA  | FA BF B5 DA       |
/// +-----------+-------------+-------------------+
/// | testnet3  | 0x0709110B  | 0B 11 09 07       |
/// +-----------+-------------+-------------------+
/// | namecoin  | 0xFEB4BEF9  | F9 BE B4 FE       |
/// +-----------+-------------+-------------------+
/// ```
/// 
#[derive(Debug, Clone)]
pub enum Magic {
    MainNet,
    TestNet,
    RegTest,
}

impl Magic {
    fn value(&self) -> &[u8; 4] {
        match *self {
            Magic::MainNet => &[ 0xD9, 0xB4, 0xBE, 0xF9 ],
            Magic::TestNet => &[ 0x07, 0x09, 0x11, 0x0B ],
            Magic::RegTest => &[ 0xDA, 0xB5, 0xBF, 0xFA ],
        }
    }
}

impl Encodable for Magic {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        let mut wire = self.value().clone();
        wire.reverse();
        w.write_all(&wire).map_err(|_| Error::MessageMagic)?;
        Ok(())
    }
}

impl Decodable for Magic {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Magic, Error> {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer).map_err(|_| Error::MessageMagic)?;
        buffer.reverse();
        match buffer {
            [ 0xD9, 0xB4, 0xBE, 0xF9 ] => Ok(Magic::MainNet),
            [ 0x07, 0x09, 0x11, 0x0B ] => Ok(Magic::TestNet),
            [ 0xDA, 0xB5, 0xBF, 0xFA ] => Ok(Magic::RegTest),
            _ => Err(Error::Magic)
        }
    }
}

/// https://en.bitcoin.it/wiki/Protocol_documentation
/// 
/// Message structure
/// ```
/// +------------+-------------+-----------+-------------------------------------------------+
/// | Field Size | Description | Data type | Comments                                        |
/// +------------+-------------+-----------+-------------------------------------------------+
/// |    4       | magic       | uint32_t  | Magic value indicating message origin network,  |
/// |            |             |           | and used to seek to next message when stream    |
/// |            |             |           | state is unknown                                |
/// +------------+-------------+-----------+-------------------------------------------------+
/// |   12       | command     | char[12]  | ASCII string identifying the packet content,    |
/// |            |             |           | NULL padded (non-NULL padding results in packet |
/// |            |             |           | rejected)                                       |
/// +------------+-------------+-----------+-------------------------------------------------+
/// |    4       | length      | uint32_t  | Length of payload in number of bytes            |
/// +------------+-------------+-----------+-------------------------------------------------+
/// |    4       | checksum    | uint32_t  | First 4 bytes of sha256(sha256(payload))        |
/// +------------+-------------+-----------+-------------------------------------------------+
/// |    ?       | payload     | uchar[]   | The actual data                                 |
/// +------------+-------------+-----------+-------------------------------------------------+
/// ```
/// 
pub struct Message {
    pub magic: Magic,
    // pub command: CommandString,
    // pub length: u32,
    // pub checksum: u32,
    pub payload: Payload
}

impl Message {

    fn checksum(payload: &Vec<u8>) -> Result<[u8; 4], ()> {

        let mut hasher = Sha256::default();
        hasher.input(payload.as_slice());
        let res1 = hasher.result_reset();
        hasher.input(res1);

        let res2 = hasher.result_reset();
        let hash = [res2[0], res2[1], res2[2], res2[3]];

        Ok(hash)
    }
}

#[derive(Debug)]
pub enum Payload {
    Version(Version),
    GetHeaders(GetHeaders),
    GetAddr(GetAddr)
}

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

impl ToString for Payload {
    fn to_string(&self) -> String {
        match self {
            Payload::Version(_) => "version",
            Payload::GetHeaders(_) => "getheaders",
            Payload::GetAddr(_) => "getaddr",
            _ => "unknown",
        }.to_owned()
    }
}

impl Payload {
    
    pub fn to_commandstring(&self) -> CommandString {
        let s = self.to_string();
        CommandString(s)
    }

    pub fn to_command(&self) -> Command {
        match self {
            Payload::Version(_) => Command::Version,
            Payload::GetHeaders(_) => Command::GetHeaders,
            Payload::GetAddr(_) => Command::GetAddr
        }
    }

}
impl Decodable for Payload {

    pub fn decode(r: Cursor<&Vec<u8>>) -> Result<Payload, Error> {

        let command = Command::from_str(c.0.as_ref()).map_err(|_| Error::CommandFromStr)?;
        let payload = match command {
            Command::Version => {
                let message = Version::decode(&mut r)?;
                Payload::Version(message)
            },
            Command::GetAddr => {
                let message = GetAddr::decode(&mut r)?;
                Payload::GetAddr(message)
            },
            Command::GetHeaders => {
                let message = GetHeaders::decode(&mut r)?;
                Payload::GetHeaders(message)
            },
        };
        Ok(payload)
    }
}
impl Encodable for Payload {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
        
        self.to_commandstring().encode(w)?;
        
        let mut buffer : Vec<u8> = Vec::new();
        match self {
            Payload::Version(ref dat) => dat.encode(&mut buffer),
            Payload::GetHeaders(ref dat) => dat.encode(&mut buffer),
            Payload::GetAddr(ref dat) => dat.encode(&mut buffer),
        }?;
        let payload_len = buffer.len() as u32;
        
        payload_len.encode(w).map_err(|_| Error::MessagePayLoadLen)?;

        let checksum : [u8; 4] = Message::checksum(&buffer).map_err(|_| Error::MessageChecksum)?;
        w.write_all(&checksum).map_err(|_| Error::MessageChecksum)?;
        w.write_all(buffer.as_ref()).map_err(|_| Error::MessagePayLoad)?;

        Ok(())
    }
}



impl Encodable for Message {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        self.magic.encode(w)?;
        self.payload.encode(w)?;
        Ok(())
    }
}

impl Decodable for Message {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Message, Error> {

        let magic = Magic::decode(r)?;
        let commandstring = CommandString::decode(r)?;
        let payload_len = u32::decode(r).map_err(|_| Error::MessagePayLoadLen)?;

        let mut checksum = [0u8; 4];
        r.read_exact(&mut checksum).map_err(|_| Error::MessageChecksum)?;

        let mut buffer : Vec<u8> = vec![0u8; payload_len as usize];
        r.read_exact(buffer.as_mut_slice()).map_err(|_| Error::MessagePayLoad)?;

        let checksum2 : [u8; 4] = Message::checksum(&buffer).map_err(|_| Error::MessageChecksum)?;
        if checksum2 != checksum {
            return Err(Error::MessageChecksumInvalid);
        }

        let r = Cursor::new(&buffer);
        let payload = Payload::decode(&commandstring, r)?;
        let result = Message {
            magic: magic,
            payload: payload
        };
        Ok(result)
    }
}
  

#[cfg(test)]
mod test {

    use crate::network::message::Magic;
    use crate::network::message::Message;
    use crate::network::command::CommandString;
    use crate::network::message::Error;
    use crate::network::message::Payload;
    use crate::encode::encode::{Encodable, Decodable};
    
    use crate::network::getaddr::GetAddr;

    use crate::utils::hexdump;


    #[test]
    fn when_encode_getaddr_message_then_same() {

        let dump = "
00000000   F9 BE B4 D9 67 65 74 61  64 64 72 00 00 00 00 00   main.getaddr....
00000010   00 00 00 00 5D F6 E0 E2                            len.checksu.data
";
        let original : Vec<u8> = hexdump::decode(dump);
        
        let payload = Payload::GetAddr(GetAddr { });
        let message = Message {
            magic: Magic::MainNet,
            payload: payload
        };

        let mut result : Vec<u8> = Vec::new();
        let encoded = message.encode(&mut result);

        assert!(encoded.is_ok());
        assert_eq!(original, result);
    }

}