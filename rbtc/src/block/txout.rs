use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::encode::varint::VarInt;
use crate::block::script;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
// CTxOut

use crate::block::script::Script;

#[derive(Debug)]
pub struct TxOut {
    pub amount: u64,
    pub script_pubkey: Script // scriptPubKey
} 

impl Decodable for Vec<TxOut> {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, Error> {

        let mut result : Vec<TxOut> = Vec::new();
        let count = VarInt::decode(r).map_err(|_| Error::OutputsCount)?;

        for _ in 0..count.0 {
            let output = TxOut::decode(r)?;
            result.push(output);
        }

        Ok(result)
    }
}

impl Decodable for TxOut {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, Error> {

        let amount = u64::decode(r).map_err(|_| Error::TxOutAmount)?;
        let script_pubkey = Script::decode(r).map_err(|_| Error::ScriptPubKey)?;

        let result = TxOut {
            amount: amount,
            script_pubkey: script_pubkey // scriptPubKey
        };
        
        Ok(result)
    }
}
