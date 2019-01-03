use crate::network::error::{DecodeError, EncodeError};

use sha2::{Sha256, Digest};

use std::fmt;
use std::io::{Read, Write};
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
    MainNet = 0xD9B4BEF9,
    TestNet = 0x0709110B,
    RegTest = 0xDAB5BFFA
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

pub trait NetworkMessage {
    fn command(&self) -> Command;
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError>;
}

pub trait Encodable {
    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError>;
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

    fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {

        let magic = self.magic.clone() as u32;
        w.write_u32::<LittleEndian>(magic).map_err(|_| EncodeError::MessageMagic)?;
        
        let mut command = format!("{:?}\0\0\0\0\0\0\0\0\0\0\0\0", 
            self.payload.command())
            .to_lowercase();
            
        command.truncate(12);
        w.write_all(command.as_bytes()).map_err(|_| EncodeError::MessageCommand)?;
        
        let mut payload : Vec<u8> = Vec::new();
        self.payload.encode(&mut payload)?;
        w.write_u8(payload.len() as u8).map_err(|_| EncodeError::MessagePayLoadLen)?;

        let checksum : [u8; 4] = checksum(&payload).map_err(|_| EncodeError::MessageChecksum)?;
        w.write_all(&checksum).map_err(|_| EncodeError::MessageChecksum)?;
        w.write_all(payload.as_ref()).map_err(|_| EncodeError::MessagePayLoad)?;

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
    use crate::network::message::EncodeError;
    use crate::network::message::Encodable;
    use crate::network::message::NetworkMessage;
    use crate::utils::hexdump;

    struct NetworkMessageTest {
        text: Vec<u8>
    }

    impl NetworkMessage for NetworkMessageTest {

        fn command(&self) -> Command {
            Command::GetHeaders
        }

        fn encode(&self, w: &mut Vec<u8>) -> Result<(), EncodeError> {
            w.append(&mut self.text.clone());
            Ok(())
        }
    }

    #[test]
    fn when_encode_message_then_same() {

        let dump = "
00000000   F9 BE B4 D9 67 65 74 68  65 61 64 65 72 73 00 00   main.getheaders.
00000010   00 5D F6 E0 E2                                     len.checksum....
";

        let original : Vec<u8> = hexdump::parse(dump);
        
        let payload = NetworkMessageTest {
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
00000010   04 F7 A3 55 C0 00 01 02  03                        len.checksu.data
";

        let original : Vec<u8> = hexdump::parse(dump);
        
        let payload = NetworkMessageTest {
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
}