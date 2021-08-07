use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
  action::Action,
  state::State,
  state_node::StateNode,
  state_node::{self, StateNodeConfig},
};

#[derive(Debug)]
pub struct MachineConfig<'mc> {
  pub id: &'mc str,
  pub states: Vec<StateNodeConfig<'mc>>,
}

pub struct Machine {
  id: String,
  root: String,
  states: HashMap<String, StateNode>,
  // actions: Vec<Box<dyn Fn()>>,
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.root)
      .field("states", &self.states)
      // .field("actions", &self.actions.len())
      .finish()
  }
}
impl Machine {
  pub fn stub() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: String::from(""),
      root: String::from(""),
      states: HashMap::new(),
    }))
  }
  pub fn new(config: MachineConfig) -> Rc<RefCell<Self>> {
    let machine = Rc::new(RefCell::new(Self {
      id: String::from(config.id),
      root: String::from(config.id),
      states: HashMap::new(),
      // actions: vec![],
    }));
    // Root node
    machine.borrow_mut().states.insert(
      String::from(config.id),
      StateNode {
        id: String::from(config.id),
        key: String::from(config.id),
        machine: Rc::clone(&machine),
        parent: None,
        kind: state_node::Kind::Atomic,
        on: HashMap::new(),
        entry: vec![],
        exit: vec![],
        done_data: None,
        transitions: vec![],
        states: HashMap::new(),
      },
    );

    let state_map = states_vec_to_map(Rc::clone(&machine), config.states, String::from(config.id));

    machine.borrow_mut().states = state_map;

    machine
  }
  pub fn transition(&self, state: State, event: &str) -> State {
    State::new()
  }
  pub fn state_from(&self, state_value: &str) -> State {
    State::new()
  }
  pub fn initial_state(&self) {}

  // Internal
  fn get_actions(&self, actions: Vec<Action>) -> (Vec<Box<dyn Fn()>>, Vec<String>) {
    (vec![Box::new(|| {})], vec![])
  }
  fn get_by_id(&self, id: &str) -> Option<&StateNode> {
    self.states.get(id)
  }
  fn get_configuration(&self, state_value: &str, parent: Option<&StateNode>) -> Vec<StateNode> {
    vec![StateNode::new()]
  }
}

fn states_vec_to_map(
  machine: Rc<RefCell<Machine>>,
  states: Vec<StateNodeConfig>,
  parent: String,
) -> HashMap<String, StateNode> {
  states.into_iter().fold(HashMap::new(), |mut acc, s| {
    let current_id = parent.clone() + "." + s.id;

    acc.insert(
      current_id.clone(),
      StateNode {
        id: current_id.clone(),
        key: String::from(s.id),
        machine: Rc::clone(&machine),
        parent: Some(parent.clone()),
        kind: state_node::Kind::Atomic,
        on: HashMap::new(),
        // Convert config/input into a list of actions
        entry: vec![],
        // Convert config/input into a list of actions
        exit: vec![],
        done_data: None,
        transitions: vec![],
        states: HashMap::new(),
      },
    );

    acc.extend(states_vec_to_map(
      Rc::clone(&machine),
      s.states,
      String::from(s.id),
    ));

    acc
  })
}
