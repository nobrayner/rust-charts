use std::collections::HashMap;
use std::fmt;

use crate::{action::Action, state::State, state_node::StateNode};

#[derive(Debug)]
pub struct MachineConfig {}

pub struct Machine {
  id: String,
  root: StateNode,
  id_map: HashMap<String, StateNode>,
  config: MachineConfig,
  states: HashMap<String, StateNode>,
  actions: Vec<Box<dyn Fn()>>,
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.root)
      .field("id_map", &self.id_map)
      .field("config", &self.config)
      .field("states", &self.states)
      .field("actions", &self.actions.len())
      .finish()
  }
}
impl Machine {
  pub fn transition(&self, state: State, event: String) -> State {
    State::new()
  }
  pub fn state_from(&self, state_value: String) -> State {
    State::new()
  }
  pub fn initial_state(&self) {}

  // Internal
  fn get_actions(&self, actions: Vec<Action>) -> (Vec<Box<dyn Fn()>>, Vec<String>) {
    (vec![Box::new(|| {})], vec![])
  }
  fn register(&self, state_node: StateNode) {}
  fn get_by_id(&self, id: &String) -> Option<&StateNode> {
    self.id_map.get(id)
  }
  fn get_configuration(&self, state_value: String, parent: Option<StateNode>) -> Vec<StateNode> {
    vec![StateNode::new()]
  }
}
// Delete when properly typed
impl Machine {
  pub fn new() -> Self {
    Self {
      id: String::from(""),
      root: StateNode::new(),
      id_map: HashMap::new(),
      config: MachineConfig {},
      states: HashMap::new(),
      actions: vec![],
    }
  }
}
