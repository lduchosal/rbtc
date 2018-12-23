use crate::bo::block::Block;
use crate::bo::transaction::Transaction;

pub fn parse(hex: &Vec<u8>) -> Block {

    let result = Block {
        version: 0,
        previous: [0; 32],
        merkleroot: [0; 32],
        time: 0,
        bits: 0,
        nonce: 0,
        transactions: Vec::new()
    };
    result
}