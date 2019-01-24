extern crate sm;

use crate::cli::*;
use crate::cli::result::*;

use std::net::{TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io::{Cursor};

use std::time;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use threadpool::ThreadPool;

use sm::NoneEvent;
use sm::sm;
use self::RbtcFsm::*;
use self::RbtcFsm::Variant;
use self::RbtcFsm::Variant::*;

sm! {

    RbtcFsm {

        // Init
        InitialStates { Init }

        // Addr
        SetAddrSucceed { Init => SetAddr }
        SetAddrFailed { Init => Init }

    }
}

enum Request {
    SetAddr(String)
}

enum Response {}


pub struct RbtcPool {
    pool: threadpool::ThreadPool,
}

impl RbtcPool {
    
    fn new() -> RbtcPool {

        let pool = ThreadPool::new(2);
        RbtcPool {
            pool: pool,
        }
    }

    fn run(&mut self) -> (mpsc::Sender<Request>, mpsc::Receviver<Request>) {

        let (send_request, rcv_request): (mpsc::Sender<Request>, mpsc::Receviver<Request>) = channel();
        let (send_response, rcv_response): (mpsc::Sender<Response>, mpsc::Receviver<Response>) = channel();

        trace!("run");
        
        let query = (send_request.clone(), rcv_response);

        self.pool.execute(move || {

            let rbtc = Rbtc::new(query);
            let mut fsm = Machine::new(Init).as_enum();
            while let Ok(request) = rcv_request.recv() {
                fsm = match request {
                    Request::SetAddr(ref addr) => rbtc.set_addr(fsm, addr),
                }
            }
        });
        
    }

    fn join(&mut self) {
        self.pool.join();
    }

}

pub struct Rbtc {

    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,

    query: (mpsc::Sender<Request>, mpsc::Receiver<Response>)
}

trait RbtcFsmEvents {

    // Init
    fn set_addr_on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>, addr: &str) -> Variant;
    fn set_addr_on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>, addr: &str) -> Variant;
}

trait RbtcInternal {
    fn do_init(&mut self) -> InitResult;
    fn do_set_addr(&mut self, addr: &str) -> SetAddrResult;
}

impl Rbtc {

    fn new(query: (mpsc::Sender<Request>, mpsc::Receiver<Response>)) -> Rbtc {

        let node_ip_port = "127.0.0.1:8333".to_string();

        Rbtc {
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            stream: None,
            query: query
        }
    }
    

    fn set_addr(&mut self, fsm: RbtcFsm::Variant, addr: &str) -> RbtcFsm::Variant {

        match fsm {
            InitialInit(m) => self.set_addr_on_init_by_none_event(m, addr),
            InitBySetAddrFailed(m) => self.set_addr_on_init_by_set_addr_failed(m, addr),
            _ => fsm,
        }
    }
}

impl RbtcFsmEvents for Rbtc  {


    fn set_addr_on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>, addr: &str) -> Variant {
        trace!("set_addr_on_init_by_none_event");

        match self.do_set_addr(addr) {
            SetAddrResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
            SetAddrResult::ParseAddrFailed => m.transition(SetAddrFailed).as_enum(),
        }
    }
    
    fn set_addr_on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>, addr: &str) -> Variant {
        trace!("set_addr_on_init_by_set_addr_failed");

        match self.do_set_addr(addr) {
            SetAddrResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
            SetAddrResult::ParseAddrFailed => m.transition(SetAddrFailed).as_enum(),
        }
    }

}


impl RbtcInternal for Rbtc {

    fn do_init(&mut self) -> InitResult {
        trace!("do_init");
        InitResult::Succeed
    }

    fn do_set_addr(&mut self, addr: &str) -> SetAddrResult {

        trace!("do_set_addr");
        debug!("do_set_addr [addr: {}]", addr);

        let mut node_ip_port = addr.to_string();

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
