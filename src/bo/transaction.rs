// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
//
// /** The basic transaction that is broadcasted on the network and contained in
//  * blocks.  A transaction can contain multiple inputs and outputs.
//  */
// class CTransaction
// {
use crate::bo::txout::TxOut;
use crate::bo::txin::TxIn;

#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub version: i32,
    pub locktime: u32
}