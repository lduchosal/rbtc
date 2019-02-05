use std::collections::HashMap;


#[derive(Eq, PartialEq, Hash, Clone)]
pub enum State {
    Init,
    Connected,
    End
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Trigger {
    ConnectParseAddrFailed,
    ConnectSucceed,
    ConnectFailed,
    ConnectRetryFailed,
    ReadSucceed,
    ReadFailed,
    ReadRetryFailed,
    WriteSucceed,
    WriteFailed,
    WriteRetryFailed,
}

pub struct StateMachine {
    state: State,
    states: HashMap<State, StateConfigurationId>,
    configurations: HashMap<StateConfigurationId, StateConfiguration>,
    ids: atomic::Atomic<u32>,
}

pub struct StateConfiguration {
    triggers: HashMap<Trigger, TriggerConfiguration>,
    entry_actions: Vec<fn() -> ()>,
    exit_actions: Vec<fn() -> ()>,
}

pub struct StateConfigurationOperation<'M> {
    id: StateConfigurationId,
    machine: &'M mut StateMachine,
}

pub struct TriggerConfiguration {
    trigger: Trigger,
    destination: State,
}

type TriggerConfigurationId = u32;
type StateConfigurationId = u32;

impl StateMachine {

    pub fn new(initial_state: State) -> StateMachine {
        StateMachine {
            state: initial_state,
            states: HashMap::new(),
            configurations: HashMap::new(),
            ids: atomic::Atomic::new(0),
        }
    }

    fn next_id(&mut self) -> u32 {
        self.ids.fetch_add(1, atomic::Ordering::Relaxed)
    }

    fn next_state_configuration_id(&mut self) -> StateConfigurationId {
        self.next_id() as StateConfigurationId
    }

    fn next_trigger_configuration_id(&mut self) -> TriggerConfigurationId {
        self.next_id() as TriggerConfigurationId
    }

    pub fn configure(&mut self, state: State) -> StateConfigurationOperation {

        let id = self.next_state_configuration_id();
        let configuration = StateConfiguration {
            triggers: HashMap::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        };

        self.states.insert(state, id);
        self.configurations.insert(id, configuration);

        StateConfigurationOperation {
            id: id,
            machine: self,
        }
    }

    pub fn fire(&mut self, trigger: Trigger) {

        // csid : current state configurationid
        let current_state = &self.state;
        let csid = self.states.get(current_state);
        if csid.is_none() {
            return;
        }
        let csid = csid.unwrap();

        // csc : current state configuration
        let csc = self.configurations.get(csid);
        if csc.is_none() {
            return;
        }
        let csc = csc.unwrap();
        
        // cstc : current state trigger configuration
        let cstc = csc.triggers.get(&trigger);
        if cstc.is_none() {
            return;
        }
        let cstc = cstc.unwrap();

        // nsid : next state id
        let next_state = cstc.destination.clone();
        let nsid = self.states.get(&next_state);
        if nsid.is_none() {
            return;
        }
        let nsid = nsid.unwrap();

        // nsc : next state configuration
        let nsc = self.configurations.get(nsid);
        if nsc.is_none() {
            return;
        }
        let nsc = nsc.unwrap();

        for on_exit in csc.exit_actions.iter() {
            on_exit();
        }

        for on_entry in nsc.entry_actions.iter() {
            on_entry();
        }
        self.state = next_state;
    }
}

impl<'M> StateConfigurationOperation<'M> {

    pub fn permit(self, trigger: Trigger, destination: State) -> Self {
        let trigger_configuration = TriggerConfiguration {
            trigger: trigger.clone(),
            destination: destination,
        };

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.triggers.insert(trigger, trigger_configuration);
        self
    }

    pub fn on_entry(self, f: fn() -> ()) -> Self {

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.entry_actions.push(f);
        self
    }

    pub fn on_exit(self, f: fn() -> ()) -> Self {

        let config = self.machine.configurations.get_mut(&self.id);
        if config.is_none() {
            return self;
        }
        let config = config.unwrap();

        config.exit_actions.push(f);
        self
    }
}
