extern crate sm;

use crate::walker::*;

use sm::NoneEvent;
use sm::sm;
use std::time;
use std::thread;

use self::WalkerSm::Variant;
use self::WalkerSm::Variant::*;
use self::WalkerSm::*;

pub(crate) trait WalkerSmEvents {
    fn run(&mut self) ;
    fn on_initial_init(&mut self, m: Machine<Init, NoneEvent>) -> Variant;
    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant;
    fn on_connect_by_connect_socket(&mut self, m: Machine<Connect, ConnectSocket>) -> Variant;
    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant;
    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant;
    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant;
    fn on_verack_sent_by_send_verack(&mut self, m: Machine<VerackSent, SendVerack>) -> Variant;
    fn on_handshake_by_set_version(&mut self, m: Machine<Handshake, SetVersion>) -> Variant;
    fn on_handshake_by_receive_other(&mut self, m: Machine<Handshake, ReceiveOther>) -> Variant;
    fn on_handshake_by_send_getaddr_failed(&mut self, m: Machine<Handshake, SendGetAddrFailed>) -> Variant;
    fn on_get_addr_by_send_getaddr(&mut self, m: Machine<GetAddr, SendGetAddr>) -> Variant;
    fn on_addr_by_receive_addr(&mut self, m: Machine<Addr, ReceiveAddr>) -> Variant;

    fn on_end_by_parse_addr_failed(&self, m: Machine<End, ParseAddrFailed>);
    fn on_end_by_parse_addr(&self, m: Machine<End, ParseAddr>);
    fn on_end_by_retry_failed(&self, m: Machine<End, RetryFailed>);
    fn on_end_by_send_version_failed(&self, m: Machine<End, SendVersionFailed>);
    fn on_end_by_receive_version_failed(&self, m: Machine<End, ReceiveVersionFailed>);
    fn on_end_by_receive_verack_failed(&self, m: Machine<End, ReceiveVerackFailed>);
    fn on_end_by_send_verack_failed(&self, m: Machine<End, SendVerackFailed>);
    fn on_end_by_send_get_addr_retry_failed(&self, m: Machine<End, SendGetAddrRetryFailed>);
}

impl WalkerSmEvents for NodeWalker  {

    fn run(&mut self) {


        trace!("run");
        let mut iteration = 0;
        let mut sm = Machine::new(Init).as_enum();

        loop {

            let sleep = time::Duration::from_millis(500);
            thread::sleep(sleep);

            debug!("run [sm: {:?}]", sm);
            debug!("run [i: {:?}]", iteration);
            debug!("run [sleep: {:?}]", sleep);

            iteration = iteration + 1;

            sm = match sm {
                InitialInit(m) => self.on_initial_init(m),
                ConnectByConnectSocket(m) => self.on_connect_by_connect_socket(m),
                InitByConnectFailed(m) => self.on_init_by_connect_failed(m),
                VersionSentBySendVersion(m) => self.on_version_sent_by_send_version(m),
                VersionReceivedByReceiveVersion(m) => self.on_version_received_by_receive_version(m),
                VerackReceivedByReceiveVerack(m) => self.on_verack_received_by_receive_verack(m),
                VerackSentBySendVerack(m) => self.on_verack_sent_by_send_verack(m),
                HandshakeBySetVersion(m) => self.on_handshake_by_set_version(m),
                HandshakeByReceiveOther(m) => self.on_handshake_by_receive_other(m),
                HandshakeBySendGetAddrFailed (m) => self.on_handshake_by_send_getaddr_failed(m),
                GetAddrBySendGetAddr(m) => self.on_get_addr_by_send_getaddr(m),
                AddrByReceiveAddr(m) => self.on_addr_by_receive_addr(m),

                EndByParseAddrFailed(m) => { self.on_end_by_parse_addr_failed(m); break; },
                EndByRetryFailed(m) => { self.on_end_by_retry_failed(m); break; },
                EndBySendVersionFailed(m) => { self.on_end_by_send_version_failed(m); break; },
                EndByReceiveVersionFailed(m) => { self.on_end_by_receive_version_failed(m); break; },
                EndByReceiveVerackFailed(m) => { self.on_end_by_receive_verack_failed(m); break; },
                EndBySendVerackFailed(m) => { self.on_end_by_send_verack_failed(m); break; },
                EndBySendGetAddrRetryFailed(m) => { self.on_end_by_send_get_addr_retry_failed(m); break; },
                EndByParseAddr(m) => { self.on_end_by_parse_addr(m); break; },
            };
        }
        debug!("run finished");
    }

    fn on_initial_init(&mut self, m: Machine<Init, NoneEvent>) -> Variant {

        trace!("on_initial_init");
        match self.init_connect_retry() {
            InitConnectResult::Succeed => m.transition(ConnectSocket).as_enum(),
            InitConnectResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            InitConnectResult::ParseAddrFailed => m.transition(ParseAddrFailed).as_enum(),
            InitConnectResult::TooManyRetry => m.transition(RetryFailed).as_enum(),
        }
    }

    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant {

        trace!("on_init_by_connect_failed");
        match self.connect_retry() {
            ConnectRetryResult::Succeed => m.transition(ConnectSocket).as_enum(),
            ConnectRetryResult::ConnectFailed => m.transition(ConnectFailed).as_enum(),
            ConnectRetryResult::TooManyRetry => m.transition(RetryFailed).as_enum(),
        }
    }

    fn on_connect_by_connect_socket(&mut self, m: Machine<Connect, ConnectSocket>) -> Variant {

        trace!("on_connect_by_connect_socket");
        match self.send_version() {
            SendMessageResult::Succeed => m.transition(SendVersion).as_enum(),
            _ => m.transition(SendVersionFailed).as_enum(),
        }
    }

    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant {

        trace!("on_version_sent_by_send_version");
        match self.receive_version() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVersion).as_enum(),
            _ => m.transition(ReceiveVersionFailed).as_enum(),
        }
    }

    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant {
        
        trace!("on_version_received_by_receive_version");
        match self.receive_verack() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVerack).as_enum(),
            _ => m.transition(ReceiveVerackFailed).as_enum(),
        }
    }

    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant {

        trace!("on_verack_received_by_receive_verack");
        match self.send_verack() {
            SendMessageResult::Succeed => m.transition(SendVerack).as_enum(),
            _ => m.transition(SendVerackFailed).as_enum(),
        }
    }

    fn on_verack_sent_by_send_verack(&mut self, m: Machine<VerackSent, SendVerack>) -> Variant {

        trace!("on_verack_sent_by_send_verack");
        self.set_version();
        m.transition(SetVersion).as_enum()
    }

    fn on_handshake_by_set_version(&mut self, m: Machine<Handshake, SetVersion>) -> Variant {

        trace!("on_handshake_by_set_version");
        match self.send_getaddr_retry() {
            SendGetAddrRetryResult::Succeed => m.transition(SendGetAddr).as_enum(),
            SendGetAddrRetryResult::Failed => m.transition(SendGetAddrFailed).as_enum(),
            SendGetAddrRetryResult::TooManyRetry => m.transition(SendGetAddrRetryFailed).as_enum(),
        }
    }

    fn on_handshake_by_receive_other(&mut self, m: Machine<Handshake, ReceiveOther>) -> Variant {

        trace!("on_handshake_by_receive_other");
        match self.send_getaddr_retry() {
            SendGetAddrRetryResult::Succeed => m.transition(SendGetAddr).as_enum(),
            SendGetAddrRetryResult::Failed => m.transition(SendGetAddrFailed).as_enum(),
            SendGetAddrRetryResult::TooManyRetry => m.transition(SendGetAddrRetryFailed).as_enum(),
        }
    }

    fn on_handshake_by_send_getaddr_failed(&mut self, m: Machine<Handshake, SendGetAddrFailed>) -> Variant {

        trace!("on_handshake_by_receive_other");
        match self.send_getaddr_retry() {
            SendGetAddrRetryResult::Succeed => m.transition(SendGetAddr).as_enum(),
            SendGetAddrRetryResult::Failed => m.transition(SendGetAddrFailed).as_enum(),
            SendGetAddrRetryResult::TooManyRetry => m.transition(SendGetAddrRetryFailed).as_enum(),
        }
    }

    fn on_get_addr_by_send_getaddr(&mut self, m: Machine<GetAddr, SendGetAddr>) -> Variant {

        trace!("on_get_addr_by_send_addr");
        match self.receive_addr() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveAddr).as_enum(),
            _ => m.transition(ReceiveOther).as_enum(),
        }
    }

    fn on_addr_by_receive_addr(&mut self, m: Machine<Addr, ReceiveAddr>) -> Variant {

        trace!("on_addr_by_receive_addr");
        self.parse_addr();
        m.transition(ParseAddr).as_enum()
    }

    fn on_end_by_parse_addr_failed(&self, _m: Machine<End, ParseAddrFailed>) {

        trace!("on_end_by_parse_addr_failed");
        self.end();
    }

    fn on_end_by_retry_failed(&self, _m: Machine<End, RetryFailed>) {

        trace!("on_end_by_retry_failed");
        self.end();
    }

    fn on_end_by_send_version_failed(&self, _m: Machine<End, SendVersionFailed>) {

        trace!("on_end_by_send_version_failed");
        self.end();
    }

    fn on_end_by_receive_version_failed(&self, _m: Machine<End, ReceiveVersionFailed>) {

        trace!("on_end_by_receive_version_failed");
        self.end();
    }

    fn on_end_by_receive_verack_failed(&self, _m: Machine<End, ReceiveVerackFailed>) {

        trace!("on_end_by_receive_verack_failed");
        self.end();
    }

    fn on_end_by_send_verack_failed(&self, _m: Machine<End, SendVerackFailed>) {

        trace!("on_end_by_send_verack_failed");
        self.end();
    }

    fn on_end_by_send_get_addr_retry_failed(&self, _m: Machine<End, SendGetAddrRetryFailed>) {

        trace!("on_end_by_send_get_addr_retry_failed");
        self.end();
    }

    fn on_end_by_parse_addr(&self, _m: Machine<End, ParseAddr>) {

        trace!("on_end_by_parse_addr");
        self.end();
    }
}

sm! {

    WalkerSm {

        InitialStates { Init }
        ParseAddrFailed { Init => End }
        RetryFailed { Init => End }
        ConnectSocket { Init => Connect }
        ConnectFailed { Init => Init }

        SendVersion { Connect => VersionSent }
        SendVersionFailed { Connect => End }

        ReceiveVersion { VersionSent => VersionReceived }
        ReceiveVersionFailed { VersionSent => End }

        ReceiveVerack { VersionReceived => VerackReceived }
        ReceiveVerackFailed { VersionReceived => End }

        SendVerack { VerackReceived => VerackSent }
        SendVerackFailed { VerackReceived => End }

        SetVersion { VerackSent => Handshake }

        SendGetAddr { Handshake => GetAddr }
        SendGetAddrFailed { Handshake => Handshake }
        SendGetAddrRetryFailed { Handshake => End }

        ReceiveAddr { GetAddr => Addr }
        ReceiveOther { Handshake, GetAddr => Handshake }

        ParseAddr { Addr => End }

    }
}

