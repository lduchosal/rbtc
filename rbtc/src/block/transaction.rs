use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::block::varint::VarInt;
use crate::block::witness;

use crate::block::txout::TxOuts;
use crate::block::txin::TxIns;
use crate::block::witness::Witnesses;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

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
// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
//
// /** The basic transaction that is broadcasted on the network and contained in
//  * blocks.  A transaction can contain multiple inputs and outputs.
//  */
// class CTransaction
// {
#[derive(Debug)]
pub struct Transaction {
    pub version: i32,
    pub flag: Option<u16>,
    pub inputs: TxIns,
    pub outputs: TxOuts,
    pub witness: Option<Witnesses>,
    pub locktime: u32
}

#[derive(Debug)]
pub struct Transactions(Vec<Transaction>);

impl Decodable for Transaction {

    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Transaction, Error> {

        let version = i32::decode(r).map_err(|_| Error::TransactionVersion)?;

        let position = r.position();
        let flag = u16::decode(r)
            .map(|v| match v { 0x0100 => Some(v), _ => None })
            .map_err(|_| Error::TransactionFlag)?;
        
        if flag.is_none() {
            r.set_position(position);
        };

        let inputs = TxIns::decode(r)?;
        let outputs = TxOuts::decode(r)?;

        let witnesses = match flag {
            Some(_) => Some(Witnesses::decode(r)?),
            _ => None
        };

        let locktime = u32::decode(r).map_err(|_| Error::TransactionLockTime)?;

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
}

impl Transactions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Transaction> {
        self.0.get(index)
    }
}

impl Decodable for Transactions {
    
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Transactions, Error> {

        let mut txs : Vec<Transaction> = Vec::new();
        let count = VarInt::decode(r).map_err(|_| Error::TransactionsCount)?;

        for _ in 0..count.0 {
            let tx = Transaction::decode(r)?;
            txs.push(tx);
        }
        let result = Transactions(txs);
        Ok(result)
    }
}