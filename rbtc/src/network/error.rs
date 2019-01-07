
#[derive(PartialEq, Debug)]
pub enum Error {

    WriteI64,
    
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
}