extern crate sm;

use crate::cli::*;
use crate::cli::rbtc::Rbtc;
use crate::cli::result::*;

use sm::NoneEvent;
use sm::sm;
use std::time;
use std::thread;

use self::RbtcFsm::Variant;
use self::RbtcFsm::Variant::*;
use self::RbtcFsm::*;

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


