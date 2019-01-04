// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
//
// /** The basic transaction that is broadcasted on the network and contained in
//  * blocks.  A transaction can contain multiple inputs and outputs.
//  */
// class CTransaction
// {
use crate::primitives::txout::TxOut;
use crate::primitives::txin::TxIn;
use crate::primitives::witness::Witness;

#[derive(Debug)]
pub struct Transaction {
    pub version: i32,
    pub flag: Option<u16>,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub witness: Option<Vec<Witness>>,
    pub locktime: u32
}