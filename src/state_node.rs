use crate::{action::Action, transition::Transition};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum Kind {
  Atomic,
  Compound,
  Parallel,
  Final,
  History,
}

#[derive(Debug)]
pub struct StateNodeConfig<'s> {
  pub id: &'s str,
  pub kind: Kind,
  pub on: Vec<(&'s str, &'s str)>,
  pub initial: Option<&'s str>,
  pub states: Vec<StateNodeConfig<'s>>,
  pub on_done: Option<&'s str>,
}

pub struct StateNode {
  pub(crate) on: HashMap<String, Vec<Transition>>,
  pub(crate) parent: Option<String>,
  pub(crate) entry: Vec<Action>,
  pub(crate) exit: Vec<Action>,
  // No idea what type this really is... It is just Optional[Dict] in the Python source
  pub(crate) done_data: Option<HashMap<String, String>>,
  pub(crate) kind: Kind,
  pub(crate) transitions: Vec<Transition>,
  pub(crate) id: String,
  pub(crate) key: String,
  // A map of node keys, to node ids: HashMap<key, id>
  pub(crate) states: HashMap<String, String>,
}
impl fmt::Debug for StateNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // f.debug_struct("StateNode")
    //   .field("id", &self.id)
    //   .field("states", &self.states)
    //   .finish()
    f.write_str(&format!("<StateNode \"{}\">", &self.id))
  }
}
impl StateNode {
  pub fn new() -> Self {
    StateNode {
      on: HashMap::new(),
      parent: None,
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
  pub fn initial(&self) -> Option<Transition> {
    Some(Transition::new())
  }
  fn get_relative(&self, target: String) -> Self {
    Self::new()
  }
}
