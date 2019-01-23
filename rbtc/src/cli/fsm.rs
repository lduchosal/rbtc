extern crate sm;

use crate::walker::*;
use crate::walker::walker::NodeWalker;
use crate::walker::result::*;

use sm::NoneEvent;
use sm::sm;
use std::time;
use std::thread;

use self::WalkerFsm::Variant;
use self::WalkerFsm::Variant::*;
use self::WalkerFsm::*;

sm! {

    RbtcCliFsm {

        // Init
        InitialStates { Init }

        // Addr
        SetAddrSuceed { Init => SetAddr }
        SetAddrFailed { Init => Init }

    }
}

pub(crate) trait RbtcCliFsmEvents {

    fn run(&mut self);

    // Init
    fn on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>) -> Variant;
    fn on_init_by_connect_failed(&mut self, m: Machine<Init, ConnectFailed>) -> Variant;

    // Connect
    fn on_connect_by_connect_socket(&mut self, m: Machine<Connect, ConnectSocket>) -> Variant;

    // Version
    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant;
    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant;

    // Verack
    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant;
    fn on_verack_sent_by_send_verack(&mut self, m: Machine<VerackSent, SendVerack>) -> Variant;
    
    // Handshake
    fn on_handshake_by_set_version(&mut self, m: Machine<Handshake, SetVersion>) -> Variant;
    fn on_handshake_by_receive_other(&mut self, m: Machine<Handshake, ReceiveOther>) -> Variant;
    fn on_handshake_by_send_getaddr_failed(&mut self, m: Machine<Handshake, SendGetAddrFailed>) -> Variant;

    // Getaddr
    fn on_get_addr_by_send_getaddr(&mut self, m: Machine<GetAddr, SendGetAddr>) -> Variant;

    // Addr
    fn on_addr_by_receive_addr(&mut self, m: Machine<Addr, ReceiveAddr>) -> Variant;

    // End
    fn on_end_by_parse_addr_failed(&mut self, m: Machine<End, ParseAddrFailed>);
    fn on_end_by_parse_addr(&mut self, m: Machine<End, ParseAddr>);
    fn on_end_by_retry_failed(&mut self, m: Machine<End, RetryFailed>);
    fn on_end_by_send_version_failed(&mut self, m: Machine<End, SendVersionFailed>);
    fn on_end_by_receive_version_failed(&mut self, m: Machine<End, ReceiveVersionFailed>);
    fn on_end_by_receive_verack_failed(&mut self, m: Machine<End, ReceiveVerackFailed>);
    fn on_end_by_send_verack_failed(&mut self, m: Machine<End, SendVerackFailed>);
    fn on_end_by_send_get_addr_retry_failed(&mut self, m: Machine<End, SendGetAddrRetryFailed>);
}

impl RbtcCliEvents for RbtcCli  {

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

                // Init
                InitialInit(m) => self.on_init_by_none_event(m),
                InitByConnectFailed(m) => self.on_init_by_connect_failed(m),

                // Connect
                ConnectByConnectSocket(m) => self.on_connect_by_connect_socket(m),

                // Version
                VersionSentBySendVersion(m) => self.on_version_sent_by_send_version(m),
                VersionReceivedByReceiveVersion(m) => self.on_version_received_by_receive_version(m),

                // Verack
                VerackReceivedByReceiveVerack(m) => self.on_verack_received_by_receive_verack(m),
                VerackSentBySendVerack(m) => self.on_verack_sent_by_send_verack(m),

                // Handshake
                HandshakeBySetVersion(m) => self.on_handshake_by_set_version(m),
                HandshakeByReceiveOther(m) => self.on_handshake_by_receive_other(m),
                HandshakeBySendGetAddrFailed(m) => self.on_handshake_by_send_getaddr_failed(m),

                // Getaddr
                GetAddrBySendGetAddr(m) => self.on_get_addr_by_send_getaddr(m),

                // Addr
                AddrByReceiveAddr(m) => self.on_addr_by_receive_addr(m),

                // End
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

    fn on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>) -> Variant {
        trace!("on_init_by_none_event");
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
            SendMessageResult::Failed  => m.transition(SendVersionFailed).as_enum(),
        }
    }

    fn on_version_sent_by_send_version(&mut self, m: Machine<VersionSent, SendVersion>) -> Variant {
        trace!("on_version_sent_by_send_version");
        match self.receive_version() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVersion).as_enum(),
            ReceiveMessageResult::Failed  => m.transition(ReceiveVersionFailed).as_enum(),
        }
    }

    fn on_version_received_by_receive_version(&mut self, m: Machine<VersionReceived, ReceiveVersion>) -> Variant {
        trace!("on_version_received_by_receive_version");
        match self.receive_verack() {
            ReceiveMessageResult::Succeed => m.transition(ReceiveVerack).as_enum(),
            ReceiveMessageResult::Failed => m.transition(ReceiveVerackFailed).as_enum(),
        }
    }

    fn on_verack_received_by_receive_verack(&mut self, m: Machine<VerackReceived, ReceiveVerack>) -> Variant {
        trace!("on_verack_received_by_receive_verack");
        match self.send_verack() {
            SendMessageResult::Succeed => m.transition(SendVerack).as_enum(),
            SendMessageResult::Failed => m.transition(SendVerackFailed).as_enum(),
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
            ReceiveMessageResult::Failed => m.transition(ReceiveOther).as_enum(),
        }
    }

    fn on_addr_by_receive_addr(&mut self, m: Machine<Addr, ReceiveAddr>) -> Variant {
        trace!("on_addr_by_receive_addr");
        self.parse_addr();
        m.transition(ParseAddr).as_enum()
    }

    fn on_end_by_parse_addr_failed(&mut self, _m: Machine<End, ParseAddrFailed>) {
        trace!("on_end_by_parse_addr_failed");
        self.end(EndResult::ParseAddrFailed);
    }

    fn on_end_by_retry_failed(&mut self, _m: Machine<End, RetryFailed>) {
        trace!("on_end_by_retry_failed");
        self.end(EndResult::RetryFailed);
    }

    fn on_end_by_send_version_failed(&mut self, _m: Machine<End, SendVersionFailed>) {
        trace!("on_end_by_send_version_failed");
        self.end(EndResult::SendVersionFailed);
    }

    fn on_end_by_receive_version_failed(&mut self, _m: Machine<End, ReceiveVersionFailed>) {
        trace!("on_end_by_receive_version_failed");
        self.end(EndResult::ReceiveVersionFailed);
    }

    fn on_end_by_receive_verack_failed(&mut self, _m: Machine<End, ReceiveVerackFailed>) {
        trace!("on_end_by_receive_verack_failed");
        self.end(EndResult::ReceiveVerackFailed);
    }

    fn on_end_by_send_verack_failed(&mut self, _m: Machine<End, SendVerackFailed>) {
        trace!("on_end_by_send_verack_failed");
        self.end(EndResult::SendVerackFailed);
    }

    fn on_end_by_send_get_addr_retry_failed(&mut self, _m: Machine<End, SendGetAddrRetryFailed>) {
        trace!("on_end_by_send_get_addr_retry_failed");
        self.end(EndResult::SendGetAddrRetryFailed);
    }

    fn on_end_by_parse_addr(&mut self, _m: Machine<End, ParseAddr>) {
        trace!("on_end_by_parse_addr");
        self.end(EndResult::ParseAddr);
    }
}


