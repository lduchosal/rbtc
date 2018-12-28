use crate::business::error::ParseError;
use crate::business::script;
use crate::business::varint;

use crate::bo::outpoint::OutPoint;
use crate::bo::txin::TxIn;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


pub(crate) fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, ParseError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| ParseError::InputsCount)?;
    for _ in 0..count {
        let input = parse_input(r)?;
        result.push(input);
    }

    Ok(result)
}

pub(crate) fn parse_input(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, ParseError> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| ParseError::TxInTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TxInIndex)?;
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = script::parse_script(r)
        .map_err(|e| {
            match e {
                ParseError::ScriptContent => ParseError::SignatureScriptContent,
                ParseError::ScriptLen => ParseError::SignatureScriptLen,
                _ => e
            }
        })?;
    let sequence = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TxInSequence)?;

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
    };
    
    Ok(result)
}
