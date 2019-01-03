
#[derive(PartialEq, Debug)]
pub enum DecodeError {
    MessageMagic,
    MessageCommand,
    MessagePayLoadLen,
    MessagePayLoad,
    MessageChecksum,

    GetHeadersVersion,
    GetHeadersLocatorsCount,
    GetHeadersLocators,
    GetHeadersLocator,
    GetHeadersStop,
}

#[derive(PartialEq, Debug)]
pub enum EncodeError {
    MessageMagic,
    MessageCommand,
    MessagePayLoadLen,
    MessagePayLoad,
    MessageChecksum,

    GetHeadersVersion,
    GetHeadersLocatorsCount,
    GetHeadersLocators,
    GetHeadersLocator,
    GetHeadersStop,
}