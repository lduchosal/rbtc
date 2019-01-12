
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

    ReadExact,

    WriteAll,
    ReadBool,
    WriteBool,

    VecContent,
    VecLen,


    Sha256Count,
    ReadSha256,
    WriteSha256,
    
    Command,
    CommandFromStr,
    CommandDecode,

    Magic,
    
    MessageNotFound,
    MessageEmpty,
    MessageNotReadFully,

    MessageMagic,
    MessageMagicReverse,
    MessagePayload,

    PingNonce,
    PongNonce,

    Payload,
    PayloadLen,
    PayloadTooSmall,
    PayloadChecksum,
    PayloadChecksumData,
    PayloadChecksumInvalid,
    PayloadUnknown,
    PayloadCommandString,

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

    TimedNetworkCount,
    TimedNetworkAddrTime,

    Service,
    ServiceMatch,
    ServiceInvalid,

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

    Script,
    Signature,
    ScriptPubKey,

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
    OutPointIndex,
    
    AlertLen,
    AlertMessage,

    AddrCount,
}