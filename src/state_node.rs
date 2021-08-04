use crate::{action::Action, machine::Machine, transition::Transition};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
enum Kind {
  Atomic,
  Compound,
  Parallel,
  Final,
}

/*
This has a fair bit of recursion inside it... The best solution is most likely to contain a map of all states,
and only store the ids (index) of the StateNodes in each strcut. This way it can be looked up, but not have to
worry about lifetimes as much (though there still needs to be a link from StateNode <-> Machine)
 */
pub struct StateNode {
  on: HashMap<String, Vec<Transition>>,
  // This should most likely be a reference,
  // as we can't own the machine in the StateNode(s).
  // Use Box for now as it is easier to type everything out for now
  machine: Box<Machine>,
  // Same here
  parent: Option<Box<StateNode>>,
  initial: Option<Transition>,
  entry: Vec<Action>,
  exit: Vec<Action>,
  // No idea what type this really is... It is just Optional[Dict] in the Python source
  done_data: Option<HashMap<String, String>>,
  kind: Kind,
  transitions: Vec<Transition>,
  pub(crate) id: String,
  key: String,
  states: HashMap<String, Box<StateNode>>,
}
impl fmt::Debug for StateNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StateNode").field("id", &self.id).finish()
  }
}
impl StateNode {
  pub fn new() -> Self {
    StateNode {
      on: HashMap::new(),
      machine: Box::new(Machine::new()),
      parent: None,
      initial: None,
      entry: vec![],
      exit: vec![],
      done_data: None,
      kind: Kind::Atomic,
      transitions: vec![],
      id: String::from(""),
      key: String::from(""),
      states: HashMap::new(),
    }
  }
  pub fn get_actions(&self /* action: ??? */) -> Action {
    Action::new()
  }
  pub fn initial(&self) -> Transition {
    Transition::new()
  }
  fn get_relative(&self, target: String) -> Self {
    Self::new()
  }
}
