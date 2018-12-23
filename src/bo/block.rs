use crate::bo::transaction::Transaction;

#[derive(Debug)]
struct Block {
    // header
    version: u32,
    previous: [u8; 32],
    merkleroot: [u8; 32],
    time: u32,
    bits: u32,
    nonce: u32,
    transactions: Vec<Transaction>
}