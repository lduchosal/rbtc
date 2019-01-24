extern crate sm;

use crate::cli::*;
use crate::cli::result::*;

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

use sm::NoneEvent;
use sm::sm;
use std::time;
use std::thread;

use self::RbtcFsm::Variant;
use self::RbtcFsm::Variant::*;
use self::RbtcFsm::*;

pub struct Rbtc {

    fsm: Variant,

    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,
    response: Vec<u8>,
}

sm! {

    RbtcFsm {

        // Init
        InitialStates { Init }

        // Addr
        SetAddrSucceed { Init => SetAddr }
        SetAddrFailed { Init => Init }

    }
}

pub(crate) trait RbtcEvents {

    // Run
    fn run(&mut self);
    fn set_addr(&mut self, addr: &str);

    // Init
    fn on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>) -> Variant;
    fn on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>) -> Variant;

    // ParseAddr
    fn on_set_addr_by_set_addr_succeed(&mut self, m: Machine<SetAddr, SetAddrSucceed>);

}


impl RbtcEvents for Rbtc  {

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
                InitBySetAddrFailed(m) => self.on_init_by_set_addr_failed(m),
                SetAddrBySetAddrSucceed(m) => { self.on_set_addr_by_set_addr_succeed(m); break; },

            };
        }
        debug!("run finished");
    }

    fn set_addr(&mut self, addr: &str) {

        trace!("set_addr");
        debug!("set_addr [node_ip_port: {}]", self.node_ip_port);
        let result = match self.sm {
            InitialInit(m) => Ok(self.do_set_addr(addr)),
            InitBySetAddrFailed(m) => Ok(self.do_set_addr(addr)),
            _ => Err(())
        };

        if let Ok(r) = result {
            match r {
                SetAddrResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
            }
        }
        debug!("run finished");
    }

    fn on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>) -> Variant {
        trace!("on_init_by_none_event");

        match self.init() {
            InitResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
        }
    }
    fn on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>) -> Variant {
        trace!("on_init_by_set_addr_failed");

        match self.init() {
            InitResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
        }
    }
    fn on_set_addr_by_set_addr_succeed(&mut self, m: Machine<SetAddr, SetAddrSucceed>) {
        trace!("on_set_addr_by_set_addr_succeed");
    }
}


impl Rbtc {

    pub fn new() -> Rbtc {

        let response = Vec::new();
        let node_ip_port = "127.0.0.1:8333".to_string();
        let fsm = Machine::new(Init).as_enum();

        Rbtc {
            fsm: fsm,
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            stream: None,
            response: response,
        }
    }

    pub(crate) fn init(&mut self) -> InitResult {
        trace!("init");
        InitResult::Succeed
    }

    pub(crate) fn do_set_addr(&mut self) -> SetAddrResult {

        trace!("set_addr");
        debug!("set_addr [node_ip_port: {}]", self.node_ip_port);

        let mut node_ip_port = self.node_ip_port.clone();

        if let Ok(addr) = node_ip_port.parse() {
            self.addr = Some(addr);
            return SetAddrResult::Succeed;
        }
        
        node_ip_port.push_str(":8333");
        match node_ip_port.parse() {
            Ok(addr) => {
                self.addr = Some(addr);
                SetAddrResult::Succeed
            },
            Err(err) => {
                warn!("set_addr [err: {}]", err);
                warn!("set_addr [node_ip_port: {}]", node_ip_port);
                SetAddrResult::ParseAddrFailed
            }
        }
    }

}
