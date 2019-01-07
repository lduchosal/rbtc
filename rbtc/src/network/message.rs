use crate::network::error::Error;
use sha2::{Sha256, Digest};

use std::fmt;
use std::io::{Write};
use byteorder::{LittleEndian, WriteBytesExt};

/// https://en.bitcoin.it/wiki/Protocol_documentation <br/>
/// Known magic values:
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

/// https://en.bitcoin.it/wiki/Protocol_documentation
/// 
/// Message structure
/// 
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
/// </pre>

pub struct Message<'a> {
    pub magic: Magic,
    // pub command: Command,
    // pub length: u32,
    // pub checksum: u32,
    pub payload: &'a NetworkMessage
}

pub trait NetworkMessage : Encodable {
    fn command(&self) -> Command;
}

pub trait Encodable {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error>;
}

pub trait Decodable : Sized {
    fn decode(r: &mut Vec<u8>) -> Result<Self, Error>;
}

#[derive(Debug)]
pub enum Command {
    Version,
    Verack,
    Addr,
    Inv,
    GetData,
    NotFound,
    GetBlocks,
    GetHeaders,
    MemPool,
    Tx,
    Block,
    Headers,
    GetAddr,
    CheckOrder,
    SubmitOrder,
    Reply,
    Ping,
    Pong,
    Reject,
    FilterLoad,
    FilterAdd,
    FilterClear,
    MerkleBlock,
    Alert,
    SendHeaders,
    FeeFilter,
    SendCmpct,
    CmpctBlock,
    GetBlockTxn,
    BlockTxn,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> crate::network::message::Encodable for Message<'a> {

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {

        self.magic.encode(w)?;
        
        let mut command = format!("{:?}\0\0\0\0\0\0\0\0\0\0\0\0", 
            self.payload.command())
            .to_lowercase();
            
        command.truncate(12);
        w.write_all(command.as_bytes()).map_err(|_| Error::MessageCommand)?;
        
        let mut payload : Vec<u8> = Vec::new();
        self.payload.encode(&mut payload)?;
        w.write_u32::<LittleEndian>(payload.len() as u32).map_err(|_| Error::MessagePayLoadLen)?;

        let checksum : [u8; 4] = checksum(&payload).map_err(|_| Error::MessageChecksum)?;
        w.write_all(&checksum).map_err(|_| Error::MessageChecksum)?;
        w.write_all(payload.as_ref()).map_err(|_| Error::MessagePayLoad)?;

        Ok(())
    }
    
}

fn checksum(payload: &Vec<u8>) -> Result<[u8; 4], ()> {

    let mut hasher = Sha256::default();
    hasher.input(payload.as_slice());
    let res1 = hasher.result_reset();
    hasher.input(res1);

    let res2 = hasher.result_reset();
    let hash = [res2[0], res2[1], res2[2], res2[3]];

    Ok(hash)
}


#[cfg(test)]
mod test {

    use crate::network::message::Magic;
    use crate::network::message::Command;
    use crate::network::message::Message;
    use crate::network::message::Error;
    use crate::network::message::{NetworkMessage, Encodable};
    
    use crate::network::getaddr::GetAddr;

    use crate::utils::hexdump;

    struct NetworkMessageMock {
        text: Vec<u8>
    }

    impl NetworkMessage for NetworkMessageMock {

        fn command(&self) -> Command {
            Command::GetHeaders
        }
    }

    impl Encodable for NetworkMessageMock {

        fn encode(&self, w: &mut Vec<u8>) -> Result<(), Error> {
            w.append(&mut self.text.clone());
            Ok(())
        }
    }

    #[test]
    fn when_encode_message_then_same() {

        let dump = "
00000000   F9 BE B4 D9 67 65 74 68  65 61 64 65 72 73 00 00   main.getheaders.
00000010   00 00 00 00 5D F6 E0 E2                            len.checksum....
";

        let original : Vec<u8> = hexdump::parse(dump);
        
        let payload = NetworkMessageMock {
            text: Vec::new()
        };

        let message = Message {
            magic: Magic::MainNet,
            payload: &payload
        };

        let mut result : Vec<u8> = Vec::new();
        let encoded = message.encode(&mut result);
        assert!(encoded.is_ok());

        assert_eq!(original, result);
    }

    #[test]
    fn when_encode_text_message_then_same() {

        let dump = "
00000000   F9 BE B4 D9 67 65 74 68  65 61 64 65 72 73 00 00   main.getheaders.
00000010   04 00 00 00 F7 A3 55 C0 00 01 02  03               len.checksu.data
";

        let original : Vec<u8> = hexdump::parse(dump);
        
        let payload = NetworkMessageMock {
            text: vec![0x00, 0x01, 0x02, 0x03]
        };

        let message = Message {
            magic: Magic::MainNet,
            payload: &payload
        };

        let mut result : Vec<u8> = Vec::new();
        let encoded = message.encode(&mut result);

        assert!(encoded.is_ok());
        assert_eq!(original, result);
    }


    #[test]
    fn when_encode_getaddr_message_then_same() {

        let dump = "
00000000   F9 BE B4 D9 67 65 74 61  64 64 72 00 00 00 00 00   main.getaddr....
00000010   00 00 00 00 5D F6 E0 E2                            len.checksu.data
";
        let original : Vec<u8> = hexdump::parse(dump);
        
        let payload = GetAddr { };

        let message = Message {
            magic: Magic::MainNet,
            payload: &payload
        };

        let mut result : Vec<u8> = Vec::new();
        let encoded = message.encode(&mut result);

        assert!(encoded.is_ok());
        assert_eq!(original, result);
    }

}