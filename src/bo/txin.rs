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
use crate::bo::script::Script;
use crate::bo::outpoint::OutPoint;

#[derive(Debug)]
pub struct TxIn {
    prevout: OutPoint,
    signature: Script, // scriptSig
    sequence: u32,
    witness: Vec<Vec<u8>>
} 