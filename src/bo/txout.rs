// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
// CTxOut

use crate::bo::script::Script;

#[derive(Debug)]
pub struct TxOut {
    value: u64,
    script: Script // scriptPubKey
} 