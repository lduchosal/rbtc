use crate::block::error::DecodeError;
use crate::block::script;
use crate::block::varint;

use crate::primitives::txout::TxOut;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub(crate) fn parse_outputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, DecodeError> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = varint::decode(r).map_err(|_| DecodeError::OutputsCount)?;

    for _ in 0..count {
        let output = parse_output(r)?;
        result.push(output);
    }

    Ok(result)
}

pub(crate) fn parse_output(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, DecodeError> {

    let amount = r.read_u64::<LittleEndian>().map_err(|_| DecodeError::TxOutAmount)?;
    let script_pubkey = script::decode(r)
        .map_err(|e| {
            match e {
                DecodeError::ScriptContent => DecodeError::ScriptPubKeyScriptContent,
                DecodeError::ScriptLen => DecodeError::ScriptPubKeyScriptLen,
                _ => e
            }
        })?;

    let result = TxOut {
        amount: amount,
        script_pubkey: script_pubkey // scriptPubKey
    };
    
    Ok(result)
}