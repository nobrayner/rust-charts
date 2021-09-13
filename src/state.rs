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
impl State {
  pub(crate) fn add_configuration(&mut self, state_id: &'static str) {
    self.configuration.push(state_id);
  }
  pub(crate) fn remove_configuration(&mut self, state_id: &'static str) {
    let maybe_state_id_index = self.configuration.iter().position(|&s| s == state_id);
    if let Some(state_id_index) = maybe_state_id_index {
      self.configuration.remove(state_id_index);
    }
  }

  pub(crate) fn update_history(
    &mut self,
    state_id: &'static str,
    configuration: Vec<&'static str>,
  ) {
    self.history.insert(state_id, configuration);
  }
}
