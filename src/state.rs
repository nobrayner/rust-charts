use std::{
  collections::{HashMap, HashSet},
  fmt,
};

use crate::{action::Action, state_node::StateNode};

pub struct State {
  configuration: HashSet<StateNode>,
  value: String,
  context: HashMap<String, String>,
  actions: Vec<Action>,
}
impl fmt::Debug for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("State")
      .field("value", &self.value)
      .field("context", &self.context)
      .field("actions", &self.actions)
      .finish()
  }
}
impl State {
  pub fn new() -> Self {
    Self {
      configuration: HashSet::new(),
      value: String::from(""),
      context: HashMap::new(),
      actions: vec![],
    }
  }
}
