// https://github.com/bitcoin/bitcoin/blob/master/src/primitives/transaction.h
// CTxOut

use crate::primitives::script::Script;

#[derive(Debug)]
pub struct TxOut {
    pub amount: u64,
    pub script_pubkey: Script // scriptPubKey
} 