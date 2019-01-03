
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

    VersionVersion,
    VersionServices,
    VersionTimestamp,
    VersionReceiver,
    VersionSender,
    VersionNonce,
    VersionUserAgent,
    VersionUserAgentLen,
    VersionStartHeight,
    VersionRelay,
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

    VersionVersion,
    VersionServices,
    VersionTimestamp,
    VersionReceiver,
    VersionSender,
    VersionNonce,
    VersionUserAgent,
    VersionUserAgentLen,
    VersionStartHeight,
    VersionRelay,
}