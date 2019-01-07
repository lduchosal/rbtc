use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::block::script;
use crate::block::varint::VarInt;

use crate::primitives::outpoint::OutPoint;
use crate::primitives::txin::TxIn;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};


pub(crate) fn decode_all(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, Error> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = VarInt::decode(r).map_err(|_| Error::InputsCount)?;
    for _ in 0..count.0 {
        let input = decode(r)?;
        result.push(input);
    }

    Ok(result)
}

pub(crate) fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, Error> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| Error::TxInTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().map_err(|_| Error::TxInIndex)?;
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = script::decode(r)
        .map_err(|e| {
            match e {
                Error::ScriptContent => Error::SignatureScriptContent,
                Error::ScriptLen => Error::SignatureScriptLen,
                _ => e
            }
        })?;
    let sequence = r.read_u32::<LittleEndian>().map_err(|_| Error::TxInSequence)?;

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
    };
    
    Ok(result)
}
