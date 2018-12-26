use crate::bo::witness::Witness;
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
    TransactionFlag,
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

    WitnessesCount,
    WitnessLen,
    WitnessData,

    VarInt,
    VarInt2,

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

/// https://en.bitcoin.it/wiki/Block
/// 
/// Block structure
/// +----------------------+------------------------------------------------------+----------------------------+
/// | Field                | Description                                          | Size                       |
/// +----------------------+------------------------------------------------------+----------------------------+
/// | Magic no             | value always 0xD9B4BEF9                              |  4 bytes                   |
/// +----------------------+------------------------------------------------------+----------------------------+
/// | Blocksize            | number of bytes following up to end of block         |  4 bytes                   |
/// +----------------------+------------------------------------------------------+----------------------------+
/// | Blockheader          | consists of 6 items                                  | 80 bytes                   |
/// +----------------------+------------------------------------------------------+----------------------------+
/// | Transaction counter  | positive integer VI = VarInt                         |  1 - 9 bytes               |
/// +----------------------+------------------------------------------------------+----------------------------+
/// | transactions         | the (non empty) list of transactions                 |  <Transaction counter>-    |
/// |                      |                                                      |  many transactions         |
/// +----------------------+------------------------------------------------------+----------------------------+
/// 

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
    let count = parse_varint(r).map_err(|_| ParseError::TransactionsCount)?;

    for _ in 0..count {
        let transaction = parse_transaction(r)?;
        result.push(transaction);
    }
    
    Ok(result)
}

/// 
/// https://en.bitcoin.it/wiki/Transaction
/// 
/// General format of a Bitcoin transaction (inside a block)
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Field           |  Description                                 |  Size                        | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Version no      |  currently 1                                 |  4 bytes                     | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Flag            |  If present, always 0001, and indicates      |  optional                    | 
/// |                 |   the presence of witness data               |  2 byte array                | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | In-counter      |  positive integer VI = VarInt                |  1 - 9 bytes                 | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | list of inputs  |  the first input of the first transaction is | <in-counter>-many inputs     | 
/// |                 |  also called "coinbase" (its content was     |                              | 
/// |                 |  ignored in earlier versions)                |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Out-counter	  |  positive integer VI = VarInt	             | 1 - 9 bytes                  | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | list of outputs |  the outputs of the first transaction spend  | <out-counter>-many outputs   | 
/// |                 |  the mined bitcoins for the block            |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | Witnesses       |  A list of witnesses, 1 for each input,      | variable                     | 
/// |                 |  omitted if flag above is missing	,        | see Segregated_Witness       | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// | lock_time       |  if non-zero and sequence numbers are        | 4 bytes                      | 
/// |                 |  < 0xFFFFFFFF: block height or timestamp     |                              | 
/// |                 |  when transaction is final                   |                              | 
/// +-----------------+----------------------------------------------+------------------------------+ 
/// 
fn parse_transaction(r: &mut Cursor<&Vec<u8>>) -> Result<Transaction, ParseError> {

    let version = r.read_i32::<LittleEndian>().map_err(|_| ParseError::TransactionVersion)?;

    let position = r.position();
    let flag = r.read_u16::<LittleEndian>()
        .map(|v| match v { 0x0100 => Some(v), _ => None })
        .map_err(|_| ParseError::TransactionFlag)?;
    
    if flag.is_none() {
        r.set_position(position);
    };

    let inputs = parse_inputs(r)?;
    let outputs = parse_outputs(r)?;

    let witnesses = match flag {
        Some(_) => Some(parse_witnesses(r)?),
        _ => None
    };

    let locktime = r.read_u32::<LittleEndian>().map_err(|_| ParseError::TransactionLockTime)?;

    let result = Transaction {
        version: version,
        flag: flag,
        inputs: inputs,
        outputs: outputs,
        witness: witnesses,
        locktime: locktime
    };
    
    Ok(result)
}


fn parse_witnesses(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Witness>, ParseError> {

    let mut result : Vec<Witness> = Vec::new();
    let count = r.read_u8().map_err(|_| ParseError::WitnessesCount)? as usize;
    for _ in 0..count {
        let witness = parse_witness(r)?;
        result.push(witness);
    }

    Ok(result)
}

/// https://en.bitcoin.it/wiki/Protocol_documentation#Transaction_Verification
/// 
/// Variable length integer
/// Integer can be encoded depending on the represented value to save space. 
/// Variable length integers always precede an array/vector of a type of data 
/// that may vary in length. Longer numbers are encoded in little endian.
/// 
/// +----------------+-----------------+----------------------------------------------+
/// | Value          | Storage length  |  Format                                      |
/// +----------------+-----------------+----------------------------------------------+
/// | < 0xFD         | 1               |  uint8_t                                     |
/// +----------------+-----------------+----------------------------------------------+
/// | <= 0xFFFF      | 3               |  0xFD followed by the length as uint16_t     |
/// +----------------+-----------------+----------------------------------------------+
/// | <= 0xFFFF FFFF | 5               |  0xFE followed by the length as uint32_t     |
/// +----------------+-----------------+----------------------------------------------+
/// | -              | 9               |  0xFF followed by the length as uint64_t     |
/// +----------------+-----------------+----------------------------------------------+
/// 
/// If you're reading the Satoshi client code (BitcoinQT) it refers to this 
/// encoding as a "CompactSize". Modern BitcoinQT also has the CVarInt class 
/// which implements an even more compact integer for the purpose of local 
/// storage (which is incompatible with "CompactSize" described here). 
/// CVarInt is not a part of the protocol.
/// 
fn parse_varint(r: &mut Cursor<&Vec<u8>>) -> Result<usize, ParseError> {

    let varlen = r.read_u8().map_err(|_| ParseError::VarInt)?;
    match varlen {
        0xFD => r.read_u16::<LittleEndian>().map(|v| v as usize),
        0xFE => r.read_u32::<LittleEndian>().map(|v| v as usize),
        0xFF => r.read_u64::<LittleEndian>().map(|v| v as usize),
        _ => Ok(varlen as usize)
    }
    .map_err(|_| ParseError::VarInt2)

}

fn parse_witness(r: &mut Cursor<&Vec<u8>>) -> Result<Witness, ParseError> {

    let varlen = parse_varint(r).map_err(|_| ParseError::WitnessLen)?;
    let mut data = vec![0u8; varlen];
    let mut data_ref = data.as_mut_slice();
    r.read_exact(&mut data_ref).map_err(|_| ParseError::WitnessData)?;

    let result = Witness {
        data: data
    };

    Ok(result)
}

fn parse_inputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxIn>, ParseError> {

    let mut result : Vec<TxIn> = Vec::new();
    let count = parse_varint(r).map_err(|_| ParseError::InputsCount)?;
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

    let result = TxIn {
        previous: previous,
        signature: signature,
        sequence: sequence,
    };
    
    Ok(result)
}

fn parse_outputs(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<TxOut>, ParseError> {

    let mut result : Vec<TxOut> = Vec::new();
    let count = parse_varint(r).map_err(|_| ParseError::OutputsCount)?;

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

    let scriptlen = parse_varint(r).map_err(|_| ParseError::ScriptLen)?;
    let mut content = vec![0u8; scriptlen];
    let mut content_ref = content.as_mut_slice();
    r.read_exact(&mut content_ref).map_err(|_| ParseError::ScriptContent)?;

    let result = Script {
        content:     content
    };

    Ok(result)
}
