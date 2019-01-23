pub enum InitResult {
    Succeed,
}

pub enum SetAddrResult {
    Succeed,
    ParseAddrFailed,
    ConnectFailed,
    TooManyRetry,
}
