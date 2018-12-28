use crate::business::error::ParseError;
use crate::business::script;
use crate::business::varint;

use crate::bo::txout::TxOut;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


pub(crate) fn parse_outputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, ParseError> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| ParseError::OutputsCount)?;

    for _ in 0..count {
        let output = parse_output(r)?;
        result.push(output);
    }

    Ok(result)
}


pub(crate) fn parse_output(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, ParseError> {

    let amount = r.read_u64::<LittleEndian>().map_err(|_| ParseError::TxOutAmount)?;
    let script_pubkey = script::parse_script(r)
        .map_err(|e| {
            match e {
                ParseError::ScriptContent => ParseError::ScriptPubKeyScriptContent,
                ParseError::ScriptLen => ParseError::ScriptPubKeyScriptLen,
                _ => e
            }
        })?;

    let result = TxOut {
        amount: amount,
        script_pubkey: script_pubkey // scriptPubKey
    };
    
    Ok(result)
}