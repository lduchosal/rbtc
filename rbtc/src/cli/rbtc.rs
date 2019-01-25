extern crate sm;

use std::sync::mpsc::SendError;
use std::sync::mpsc::RecvError;
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

enum Response {
    SetAddr(bool)
}


pub struct RbtcPool {
    pool: threadpool::ThreadPool,
    send: mpsc::Sender<Request>,
    recv: mpsc::Receiver<Response>,
}

impl RbtcPool {
    
    pub fn new() -> RbtcPool {

        trace!("new");

        let pool = ThreadPool::new(2);
 
        let (send_request, rcv_request) = channel::<Request>();
        let (send_response, rcv_response) = channel::<Response>();

        pool.execute(move || {

            println!("pool.execute");

            let mut rbtc = Rbtc::new(rcv_request, send_response);
            let mut fsm = Machine::new(Init).as_enum();
            while let Ok(request) = rbtc.recv() {

                println!("rbtc.recv");
                fsm = match request {
                    Request::SetAddr(ref addr) => rbtc.set_addr(fsm, addr),
                }
            }
        });

        RbtcPool {
            pool: pool,
            send: send_request,
            recv: rcv_response
        }
    }

    pub fn set_addr(&mut self, addr: String) {

        println!("set_addr");
        let request = Request::SetAddr(addr);
        self.send.send(request);
    }

    pub fn join(&mut self) {
        self.pool.join();
    }

}

pub struct Rbtc {

    node_ip_port: String,
    
    connect_retry: u8,
    getaddr_retry: u8,

    addr: Option<SocketAddr>,
    stream: Option<TcpStream>,

    recv: mpsc::Receiver<Request>,
    send: mpsc::Sender<Response>,
}

trait RbtcFsmEvents {

    // Init
    

    fn set_addr<E> (&mut self, m: Machine<Init, E>, addr: &str) -> (Variant, SetAddrResult) where E: cli::rbtc::sm::Event;
    fn set_addr_on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>, addr: &str) -> (Variant, SetAddrResult);
    fn set_addr_on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>, addr: &str) -> (Variant, SetAddrResult);
}

trait RbtcInternal {
    fn do_init(&mut self) -> InitResult;
    fn do_set_addr(&mut self, addr: &str) -> SetAddrResult;
}

impl Rbtc {

    fn recv(&self) -> Result<Request, RecvError> {
        self.recv.recv()
    }

    fn send(&self, response: Response) -> Result<(), SendError<Response>> {
        self.send.send(response)
    }

    fn new(recv: mpsc::Receiver<Request>, send: mpsc::Sender<Response>) -> Rbtc {
        println!("new");

        let node_ip_port = "127.0.0.1:8333".to_string();

        Rbtc {
            connect_retry: 0,
            getaddr_retry: 0,
            node_ip_port: node_ip_port,
            addr: None,
            stream: None,
            recv: recv,
            send: send
        }
    }
    

    fn set_addr(&mut self, fsm: RbtcFsm::Variant, addr: &str) -> RbtcFsm::Variant {
        println!("set_addr");

        let (variant, result) = match fsm {
            InitialInit(m) => self.set_addr_on_init_by_none_event(m, addr),
            InitBySetAddrFailed(m) => self.set_addr_on_init_by_set_addr_failed(m, addr),
            _ => (fsm, SetAddrResult::InvalidState),
        };

        let succeed = match result {
            SetAddrResult::Succeed => true,
            SetAddrResult::ParseAddrFailed => false,
            SetAddrResult::InvalidState => false,
        };

        let response = Response::SetAddr(succeed);
        self.send(response);

        variant
    }
}

impl RbtcFsmEvents for Rbtc  {


    fn set_addr_on_init_by_none_event(&mut self, m: Machine<Init, NoneEvent>, addr: &str) -> (Variant, SetAddrResult)  {
        println!("set_addr_on_init_by_none_event");

        let result = self.do_set_addr(addr);
        let transition = match result {
            SetAddrResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
            SetAddrResult::ParseAddrFailed => m.transition(SetAddrFailed).as_enum(),
            SetAddrResult::InvalidState => m.transition(SetAddrFailed).as_enum(),
        }

        (transition, result)
    }
    
    fn set_addr_on_init_by_set_addr_failed(&mut self, m: Machine<Init, SetAddrFailed>, addr: &str) -> (Variant, SetAddrResult) {
        trace!("set_addr_on_init_by_set_addr_failed");

        let result = self.do_set_addr(addr);
        let transition = match result {
            SetAddrResult::Succeed => m.transition(SetAddrSucceed).as_enum(),
            SetAddrResult::ParseAddrFailed => m.transition(SetAddrFailed).as_enum(),
            SetAddrResult::InvalidState => m.transition(SetAddrFailed).as_enum(),
        }

        (transition, result)
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
                trace!("do_set_addr [ok]");
                self.addr = Some(addr);
                SetAddrResult::Succeed
            },
            Err(err) => {
                warn!("do_set_addr [err: {}]", err);
                warn!("do_set_addr [node_ip_port: {}]", node_ip_port);
                SetAddrResult::ParseAddrFailed
            }
        }
    }

}
