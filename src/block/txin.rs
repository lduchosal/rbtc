use crate::block::error::DecodeError;
use crate::block::script;
use crate::block::varint;

use crate::primitives::outpoint::OutPoint;
use crate::primitives::txin::TxIn;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


pub(crate) fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, DecodeError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = varint::decode(r).map_err(|_| DecodeError::InputsCount)?;
    for _ in 0..count {
        let input = parse_input(r)?;
        result.push(input);
    }

    Ok(result)
}

pub(crate) fn parse_input(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, DecodeError> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| DecodeError::TxInTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::TxInIndex)?;
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = script::decode(r)
        .map_err(|e| {
            match e {
                DecodeError::ScriptContent => DecodeError::SignatureScriptContent,
                DecodeError::ScriptLen => DecodeError::SignatureScriptLen,
                _ => e
            }
        })?;
    let sequence = r.read_u32::<LittleEndian>().map_err(|_| DecodeError::TxInSequence)?;

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
    };
    
    Ok(result)
}
