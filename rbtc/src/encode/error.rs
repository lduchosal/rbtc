
#[derive(PartialEq, Debug)]
pub enum Error {

    ReadI64,
    ReadI32,
    ReadI16,
    ReadI8,

    ReadU64,
    ReadU32,
    ReadU16,
    ReadU8,

    WriteI64,
    WriteI32,
    WriteI16,
    WriteI8,
        
    WriteU64,
    WriteU32,
    WriteU16,
    WriteU8,

    ReadBool,
    WriteBool,

    ReadSha256,
    WriteSha256,

    MessageMagic,
    MessageMagicReverse,
    MessagePayLoadLen,
    MessagePayLoad,
    MessageChecksum,

    GetHeadersVersion,
    GetHeadersLocatorsCount,
    GetHeadersLocators,
    GetHeadersLocator,
    GetHeadersStop,

    VersionVersion,
    VersionServices,
    VersionTimestamp,
    VersionReceiver,
    VersionSender,
    VersionNonce,
    VersionUserAgent,
    VersionUserAgentDecode,
    VersionUserAgentLen,
    VersionStartHeight,
    VersionRelay,

    NetworkAddrTime,
    NetworkAddrServices,
    NetworkAddrIp,
    NetworkAddrPort,

    TimedNetworkAddrTime,

    Service,
    ServiceMatch,

    IpAddr,
    IpAddrB1,
    IpAddrB2,
    IpAddrB3,
    IpAddrB4,
    IpAddrB5,
    IpAddrB6,
    IpAddrB7,
    IpAddrB8,


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
    TxInOutPoint,

    WitnessesCount,
    WitnessLen,
    WitnessData,

    VarInt,
    VarIntFD,
    VarIntFE,
    VarIntFF,

    OutPointTransactionHash,
    OutPointIndex
}