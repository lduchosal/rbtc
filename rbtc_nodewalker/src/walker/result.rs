
#[derive(Debug, Clone)]
pub enum EndResult {
    ParseAddrFailed,
    RetryFailed,
    SendVersionFailed,
    ReceiveVersionFailed,
    ReceiveVerackFailed,
    SendVerackFailed,
    SendGetAddrRetryFailed,
    ParseAddr,
}

pub enum ConnectResult {
    Succeed,
    ConnectFailed,
}

pub enum InitResult {
    Succeed,
    ParseAddrFailed,
}

pub enum InitConnectResult {
    Succeed,
    ParseAddrFailed,
    ConnectFailed,
    TooManyRetry,
}

pub enum SendResult {
    Succeed,
    EncodeFailed,
    WriteFailed,
}

pub enum SendMessageResult {
    Succeed,
    Failed,
}

pub enum ConnectRetryResult {
    Succeed,
    ConnectFailed,
    TooManyRetry,
}

pub enum ReceiveResult {
    ReadFailed,
    ReadEmpty,
    ReadSome,
}

pub enum DecodeResult {
    NeedMoreData,
    DecodeFailed,
    Succeed
}

pub enum ReceiveMessageResult {
    Failed,
    Succeed
}

pub enum SendGetAddrRetryResult {
    TooManyRetry,
    Succeed,
    Failed
}
