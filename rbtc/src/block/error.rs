
#[derive(PartialEq, Debug)]
pub enum Error {
 
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
    VarIntFD,
    VarIntFE,
    VarIntFF,

}
