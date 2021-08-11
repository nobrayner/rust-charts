use std::{collections::HashMap, fmt};

use crate::action::Action;

pub struct State {
  pub value: Vec<String>,
  pub context: HashMap<String, String>,
  // actions: Vec<Action>,
}
impl fmt::Debug for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("State")
      .field("value", &self.value)
      .field("context", &self.context)
      // .field("actions", &self.actions)
      .finish()
  }
}
impl State {
  pub fn stub() -> Self {
    Self {
      value: vec![],
      context: HashMap::new(),
      // actions: vec![],
    }
  }
}
