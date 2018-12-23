use crate::bo::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    // header
    pub version: u32,
    pub previous: [u8; 32],
    pub merkleroot: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub transactions: Vec<Transaction>
}