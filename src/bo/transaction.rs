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
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    version: i32,
    locktime: u32
}