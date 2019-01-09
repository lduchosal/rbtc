use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};
use crate::encode::varint::VarInt;
use crate::block::script;

use crate::block::outpoint::OutPoint;
use crate::block::script::Script;

use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h

// /** An input of a transaction.  It contains the location of the previous
//  * transaction's output that it claims and a signature that matches the
//  * output's public key.
//  *
// class CTxIn
// {
// public:
//     COutPoint prevout;
//     CScript scriptSig;
//     uint32_t nSequence;
//     CScriptWitness scriptWitness; //!< Only serialized through CTransaction


#[derive(Debug)]
pub struct TxIn {
    pub previous: OutPoint,
    pub signature: Script, // scriptSig
    pub sequence: u32,
} 

#[derive(Debug)]
pub struct TxIns (Vec<TxIn>);

impl TxIns {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, index: usize) -> Option<&TxIn> {
        self.0.get(index)
    }
}


impl Decodable for TxIns {
    
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<TxIns, Error> {

        let mut txins : Vec<TxIn> = Vec::new();
        let count = VarInt::decode(r).map_err(|_| Error::InputsCount)?;
        for _ in 0..count.0 {
            let input = TxIn::decode(r)?;
            txins.push(input);
        }
        let result = TxIns(txins);
        Ok(result)
    }
}


impl Decodable for TxIn {
    
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, Error> {

        let previous = OutPoint::decode(r).map_err(|_| Error::TxInOutPoint)?;
        let signature = Script::decode(r).map_err(|_| Error::Signature)?;
        let sequence = u32::decode(r).map_err(|_| Error::TxInSequence)?;

        let result = TxIn {
            previous: previous,
            signature: signature,
            sequence: sequence,
        };
        
        Ok(result)
    }
}
