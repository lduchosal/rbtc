use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::block::script;
use crate::block::varint::VarInt;

use crate::primitives::txout::TxOut;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};


pub(crate) fn decode_all(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, Error> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = VarInt::decode(r).map_err(|_| Error::OutputsCount)?;

    for _ in 0..count.0 {
        let output = decode(r)?;
        result.push(output);
    }

    Ok(result)
}

pub(crate) fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, Error> {

    let amount = r.read_u64::<LittleEndian>().map_err(|_| Error::TxOutAmount)?;
    let script_pubkey = script::decode(r)
        .map_err(|e| {
            match e {
                Error::ScriptContent => Error::ScriptPubKeyScriptContent,
                Error::ScriptLen => Error::ScriptPubKeyScriptLen,
                _ => e
            }
        })?;

    let result = TxOut {
        amount: amount,
        script_pubkey: script_pubkey // scriptPubKey
    };
    
    Ok(result)
}