// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
// CTxOut

use crate::bo::script::Script;

#[derive(Debug)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Script // scriptPubKey
} 