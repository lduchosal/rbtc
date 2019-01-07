
#[derive(PartialEq, Debug)]
pub enum Error {
    MessageMagic,
    MessageMagicReverse,
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

    NetworkAddrTime,
    NetworkAddrServices,
    NetworkAddrIp,
    NetworkAddrPort,

    TimedNetworkAddrTime,

    Service,
    IpAddr
}