use crate::bo::txout::TxOut;
use crate::bo::script::Script;
use crate::bo::outpoint::OutPoint;
use crate::bo::txin::TxIn;
use crate::bo::block::Block;
use crate::bo::transaction::Transaction;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub enum ParseError {
    Previous,
    MerkleRoot,
    PreviousTransactionHash,
    SignatureContent
}

pub fn parse(hex: &Vec<u8>) -> Result<Block, ParseError> {

    let mut r = Cursor::new(hex);
    let result = parse_block(&mut r)?;
    Ok(result)
}

fn parse_block(r: &mut Cursor<&Vec<u8>>) -> Result<Block, ParseError> {

    let version = r.read_u32::<LittleEndian>().unwrap();

    let mut previous = [0; 32];
    r.read_exact(&mut previous).map_err(|_| ParseError::Previous)?;

    let mut merkleroot = [0; 32];
    r.read_exact(&mut merkleroot).map_err(|_| ParseError::MerkleRoot)?;

    let time = r.read_u32::<LittleEndian>().unwrap();
    let bits = r.read_u32::<LittleEndian>().unwrap();
    let nonce = r.read_u32::<LittleEndian>().unwrap();

    let transactions = parse_transactions(r).ok().unwrap();

    let result = Block {
        version: version,
        previous: previous,
        merkleroot: merkleroot,
        time: time,
        bits: bits,
        nonce: nonce,
        transactions: transactions
    };

    Ok(result)
}

fn parse_transactions(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Transaction>, ParseError> {

    let mut result : Vec<Transaction> = Vec::new();
    let count = r.read_u8().unwrap();
    for _ in 0..count {
        let transaction = parse_transaction(r).ok().unwrap();
        result.push(transaction);
    }
    
    Ok(result)
}

fn parse_transaction(r: &mut Cursor<&Vec<u8>>) -> Result<Transaction, ParseError> {

    let version = r.read_i32::<LittleEndian>().unwrap();
    let inputs = parse_inputs(r).ok().unwrap();
    let outputs = parse_outputs(r).ok().unwrap();
    let result = Transaction {
        inputs: inputs,
        outputs: outputs,
        version: version,
        locktime: 0
    };
    
    Ok(result)
}

fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, ParseError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = r.read_u8().unwrap();
    for _ in 0..count {
        let input = parse_input(r).ok().unwrap();
        result.push(input);
    }

    Ok(result)
}

fn parse_input(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, ParseError> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| ParseError::PreviousTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().unwrap();
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = parse_script(r).ok().unwrap();
    let sequence = r.read_u32::<LittleEndian>().unwrap();
    let witness = Vec::new();

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
        witness: witness
    };
    
    Ok(result)
}

fn parse_outputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, ParseError> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = r.read_u8().unwrap();
    for _ in 0..count {
        let output = parse_output(r).ok().unwrap();
        result.push(output);
    }

    Ok(result)
}

fn parse_output(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, ParseError> {

    let amount = r.read_u64::<LittleEndian>().unwrap();
    let script_pubkey = parse_script(r).ok().unwrap();

    let result = TxOut {
        amount: amount,
        script_pubkey: script_pubkey // scriptPubKey
    };
    
    Ok(result)
}

fn parse_script(r: &mut Cursor<&Vec<u8>>) -> Result<Script, ParseError> {

    let scriptlen = r.read_u8().unwrap();
    let mut content = vec![0u8; scriptlen as usize];
    let mut content_ref = content.as_mut_slice();
    r.read_exact(&mut content_ref).map_err(|_| ParseError::SignatureContent)?;

    let result = Script {
        content: content
    };

    Ok(result)
}
