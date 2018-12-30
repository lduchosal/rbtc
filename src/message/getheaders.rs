use crate::utils::sha256::Sha256;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

/// The `getheaders` message
/// https://github.com/rust-bitcoin/rust-bitcoin/blob/45140a3251d9eca8d17baf7a4e900a4ac5baae3b/src/network/message_blockdata.rs
#[derive(Debug)]
pub struct GetHeadersMessage {
    /// The protocol version
    pub version: u32,
    /// Locator hashes --- ordered newest to oldest. The remote peer will
    /// reply with its longest known chain, starting from a locator hash
    /// if possible and block 1 otherwise.
    pub locator_hashes: Vec<Sha256>,
    /// References the header to stop at, or zero to just fetch the maximum 2000 headers
    pub stop_hash: Sha256
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    Version,
    RemainingContent,
}

fn parse_getheaders(r: &mut Cursor<&Vec<u8>>) -> Result<GetHeadersMessage, ParseError> {

    let version = r.read_u32::<LittleEndian>().map_err(|_| ParseError::Version)?;
    let locator_hashes : Vec<Sha256> = Vec::new();
    let stop_hash = Sha256 {
        hash: [0; 32]
    };

    let result = GetHeadersMessage {
        version: version,
        locator_hashes: locator_hashes,
        stop_hash: stop_hash
    };

    Ok(result)
}
