use crate::block::error::EncodeError;
use crate::block::script;
use crate::block::varint;

use crate::primitives::outpoint::OutPoint;
use crate::primitives::txin::TxIn;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


pub(crate) fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, EncodeError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| EncodeError::InputsCount)?;
    for _ in 0..count {
        let input = parse_input(r)?;
        result.push(input);
    }

    Ok(result)
}

pub(crate) fn parse_input(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, EncodeError> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| EncodeError::TxInTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().map_err(|_| EncodeError::TxInIndex)?;
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = script::parse_script(r)
        .map_err(|e| {
            match e {
                EncodeError::ScriptContent => EncodeError::SignatureScriptContent,
                EncodeError::ScriptLen => EncodeError::SignatureScriptLen,
                _ => e
            }
        })?;
    let sequence = r.read_u32::<LittleEndian>().map_err(|_| EncodeError::TxInSequence)?;

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
    };
    
    Ok(result)
}
