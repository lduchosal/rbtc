use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
//
// /** An outpoint - a combination of a transaction hash and an index n into its vout */
// class COutPoint
// {
// public:
//     uint256 hash;
//     uint32_t n;

#[derive(Debug)]
pub struct OutPoint {
    pub transaction_hash: [u8; 32],
    pub index: u32,
}

impl Decodable for OutPoint {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<OutPoint, Error> {
        
        trace!("decode");

        let transaction_hash = <[u8; 32]>::decode(r).map_err(|_| Error::OutPointTransactionHash)?;
        let index = u32::decode(r).map_err(|_| Error::OutPointIndex)?;

        let result = OutPoint {
            transaction_hash: transaction_hash,
            index: index,
        };

        Ok(result)
    }
}