use crate::block::error::Error;
use crate::block::script;
use crate::block::varint;

use crate::primitives::txout::TxOut;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub(crate) fn decode_all(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, Error> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = varint::decode(r).map_err(|_| Error::OutputsCount)?;

    for _ in 0..count {
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