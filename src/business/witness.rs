use crate::business::error::ParseError;
use crate::business::varint;

use crate::bo::witness::Witness;

use std::io::Read;
use std::io::Cursor;

pub(crate) fn parse_witnesses(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Witness>, ParseError> {

    let mut result : Vec<Witness> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| ParseError::WitnessesCount)?;
    for _ in 0..count {
        let witness = parse_witness(r)?;
        result.push(witness);
    }

    Ok(result)
}

pub(crate) fn parse_witness(r: &mut Cursor<&Vec<u8>>) -> Result<Witness, ParseError> {

    let varlen = varint::parse_varint(r).map_err(|_| ParseError::WitnessLen)?;
    let mut data = vec![0u8; varlen];
    let mut data_ref = data.as_mut_slice();
    r.read_exact(&mut data_ref).map_err(|_| ParseError::WitnessData)?;

    let result = Witness {
        data: data
    };

    Ok(result)
}

