use std::{collections::HashMap, fmt};

use crate::action::Action;

pub struct State {
  // pub value: Vec<&'static str>,
  pub configuration: Vec<&'static str>,
  // pub context: HashMap<String, String>,
  pub actions: Vec<&'static Action>,
  pub history: HashMap<&'static str, Vec<&'static str>>,
}
impl fmt::Debug for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("State")
      // .field("value", &self.value)
      .field("configuration", &self.configuration)
      // .field("context", &self.context)
      .field("actions", &self.actions)
      .finish()
  }
}
