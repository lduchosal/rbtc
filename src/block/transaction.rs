use crate::block::error::ParseError;
use crate::block::varint;
use crate::block::txin;
use crate::block::txout;
use crate::block::witness;

use crate::primitives::transaction::Transaction;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

/// 
/// https://en.bitcoin.it/wiki/Transaction
/// 
/// General format of a Bitcoin transaction (inside a block)
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Field           |  Description                                 |  Size                        | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Version no      |  currently 1                                 |  4 bytes                     | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Flag            |  If present, always 0001, and indicates      |  optional                    | 
/// |                 |   the presence of witness data               |  2 byte array                | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | In-counter      |  positive integer VI = VarInt                |  1 - 9 bytes                 | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | list of inputs  |  the first input of the first transaction is | <in-counter>-many inputs     | 
/// |                 |  also called "coinbase" (its content was     |                              | 
/// |                 |  ignored in earlier versions)                |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Out-counter	  |  positive integer VI = VarInt	             | 1 - 9 bytes                  | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | list of outputs |  the outputs of the first transaction spend  | <out-counter>-many outputs   | 
/// |                 |  the mined bitcoins for the block            |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Witnesses       |  A list of witnesses, 1 for each input,      | variable                     | 
/// |                 |  omitted if flag above is missing	,        | see Segregated_Witness       | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | lock_time       |  if non-zero and sequence numbers are        | 4 bytes                      | 
/// |                 |  < 0xFFFFFFFF: block height or timestamp     |                              | 
/// |                 |  when transaction is final                   |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// 
pub fn parse_transaction(r: &mut Cursor<&Vec<u8>>) -> Result<Transaction, ParseError> {

    let version = r.read_i32::<LittleEndian>().map_err(|_| ParseError::TransactionVersion)?;

    let position = r.position();
    let flag = r.read_u16::<LittleEndian>()
        .map(|v| match v { 0x0100 => Some(v), _ => None })
        .map_err(|_| ParseError::TransactionFlag)?;
    
    if flag.is_none() {
        r.set_position(position);
    };

    let inputs = txin::parse_inputs(r)?;
    let outputs = txout::parse_outputs(r)?;

    let witnesses = match flag {
        Some(_) => Some(witness::parse_witnesses(r)?),
        _ => None
    };

    let locktime = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TransactionLockTime)?;

    let result = Transaction {
        version: version,
        flag: flag,
        inputs: inputs,
        outputs: outputs,
        witness: witnesses,
        locktime: locktime
    };
    
    Ok(result)
}


pub(crate) fn parse_transactions(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Transaction>, ParseError> {

    let mut result : Vec<Transaction> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| ParseError::TransactionsCount)?;

    for _ in 0..count {
        let transaction = parse_transaction(r)?;
        result.push(transaction);
    }
    
    Ok(result)
}
