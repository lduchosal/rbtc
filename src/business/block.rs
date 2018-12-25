use crate::bo::txout::TxOut;
use crate::bo::script::Script;
use crate::bo::outpoint::OutPoint;
use crate::bo::txin::TxIn;
use crate::bo::block::Block;
use crate::bo::transaction::Transaction;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(PartialEq, Debug)]
pub enum ParseError {
    
    InvalidLength,
    RemainingContent,

    BlockVersion,
    BlockPrevious,
    BlockMerkleRoot,
    BlockTime,
    BlockNonce,
    BlockBits,

    TransactionsCount,
    TransactionVersion,
    TransactionLockTime,

    ScriptContent,
    ScriptLen,
    SignatureScriptContent,
    SignatureScriptLen,
    ScriptPubKeyScriptContent,
    ScriptPubKeyScriptLen,

    OutputsCount,
    TxOutAmount,

    InputsCount,
    TxInTransactionHash,
    TxInSequence,
    TxInIndex,
}

pub fn parse(hex: &Vec<u8>) -> Result<Block, ParseError> {

    if hex.len() < 81 { // might not be true
        return Err(ParseError::InvalidLength);
    }
    let mut r = Cursor::new(hex);
    let result = parse_block(&mut r)?;

    if r.position() as usize != hex.len() {
        return Err(ParseError::RemainingContent);
    }
    Ok(result)
}

fn parse_block(r: &mut Cursor<&Vec<u8>>) -> Result<Block, ParseError> {

    let version = r.read_u32::<LittleEndian>().map_err(|_| ParseError::BlockVersion)?;

    let mut previous = [0; 32];
    r.read_exact(&mut previous).map_err(|_| ParseError::BlockPrevious)?;

    let mut merkleroot = [0; 32];
    r.read_exact(&mut merkleroot).map_err(|_| ParseError::BlockMerkleRoot)?;

    let time = r.read_u32::<LittleEndian>().map_err(|_| ParseError::BlockTime)?;
    let bits = r.read_u32::<LittleEndian>().map_err(|_| ParseError::BlockBits)?;
    let nonce = r.read_u32::<LittleEndian>().map_err(|_| ParseError::BlockNonce)?;

    let transactions = parse_transactions(r)?;

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
    let count = r.read_u8().map_err(|_| ParseError::TransactionsCount)?;
    for _ in 0..count {
        let transaction = parse_transaction(r)?;
        result.push(transaction);
    }
    
    Ok(result)
}

fn parse_transaction(r: &mut Cursor<&Vec<u8>>) -> Result<Transaction, ParseError> {

    let version = r.read_i32::<LittleEndian>().map_err(|_| ParseError::TransactionVersion)?;
    let inputs = parse_inputs(r)?;
    let outputs = parse_outputs(r)?;
    let locktime = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TransactionLockTime)?;

    let result = Transaction {
        inputs: inputs,
        outputs: outputs,
        version: version,
        locktime: locktime
    };
    
    Ok(result)
}

fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, ParseError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = r.read_u8().map_err(|_| ParseError::InputsCount)?;
    for _ in 0..count {
        let input = parse_input(r)?;
        result.push(input);
    }

    Ok(result)
}

fn parse_input(r: &mut Cursor<&Vec<u8>>) -> Result<TxIn, ParseError> {

    let mut transaction_hash = [0; 32];
    r.read_exact(&mut transaction_hash).map_err(|_| ParseError::TxInTransactionHash)?;
    let index = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TxInIndex)?;
    let previous = OutPoint {
        transaction_hash: transaction_hash,
        index: index,
    };

    let signature = parse_script(r)
        .map_err(|e| {
            match e {
                ParseError::ScriptContent => ParseError::SignatureScriptContent,
                ParseError::ScriptLen => ParseError::SignatureScriptLen,
                _ => e
            }
        })?;
    let sequence = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TxInSequence)?;
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
    let count = r.read_u8().map_err(|_| ParseError::OutputsCount)?;
    for _ in 0..count {
        let output = parse_output(r)?;
        result.push(output);
    }

    Ok(result)
}

fn parse_output(r: &mut Cursor<&Vec<u8>>) -> Result<TxOut, ParseError> {

    let amount = r.read_u64::<LittleEndian>().map_err(|_| ParseError::TxOutAmount)?;
    let script_pubkey = parse_script(r)
        .map_err(|e| {
            match e {
                ParseError::ScriptContent => ParseError::ScriptPubKeyScriptContent,
                ParseError::ScriptLen => ParseError::ScriptPubKeyScriptLen,
                _ => e
            }
        })?;

    let result = TxOut {
        amount: amount,
        script_pubkey: script_pubkey // scriptPubKey
    };
    
    Ok(result)
}

fn parse_script(r: &mut Cursor<&Vec<u8>>) -> Result<Script, ParseError> {

    let scriptlen = r.read_u8().map_err(|_| ParseError::ScriptLen)?;
    let mut content = vec![0u8; scriptlen as usize];
    let mut content_ref = content.as_mut_slice();
    r.read_exact(&mut content_ref).map_err(|_| ParseError::ScriptContent)?;

    let result = Script {
        content: content
    };

    Ok(result)
}
