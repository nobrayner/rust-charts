use std::{collections::HashMap, fmt};

pub struct State {
  // pub value: Vec<&'static str>,
  pub configuration: Vec<String>,
  // pub context: Context,
  pub actions: Vec<String>,
  pub history: HashMap<String, Vec<String>>,
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
  pub(crate) fn add_configuration(&mut self, state_id: String) {
    self.configuration.push(state_id);
  }
  pub(crate) fn remove_configuration(&mut self, state_id: &str) {
    let maybe_state_id_index = self.configuration.iter().position(|s| s == state_id);
    if let Some(state_id_index) = maybe_state_id_index {
      self.configuration.remove(state_id_index);
    }
  }

  pub(crate) fn add_action(&mut self, action_id: &str) {
    self.actions.push(String::from(action_id));
  }

  pub(crate) fn update_history(&mut self, state_id: &str, configuration: Vec<String>) {
    self.history.insert(String::from(state_id), configuration);
  }
}
