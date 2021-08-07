use crate::{action::Action, machine::Machine, transition::Transition};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug)]
pub enum Kind {
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
  pub(crate) on: HashMap<String, Vec<Transition>>,
  pub(crate) machine: Rc<RefCell<Machine>>,
  pub(crate) parent: Option<String>,
  pub(crate) initial: Option<Transition>,
  pub(crate) entry: Vec<Action>,
  pub(crate) exit: Vec<Action>,
  // No idea what type this really is... It is just Optional[Dict] in the Python source
  pub(crate) done_data: Option<HashMap<String, String>>,
  pub(crate) kind: Kind,
  pub(crate) transitions: Vec<Transition>,
  pub(crate) id: String,
  pub(crate) key: String,
  pub(crate) states: HashMap<String, String>,
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
      machine: Machine::stub(),
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
