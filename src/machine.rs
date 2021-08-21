use std::{
  collections::{HashMap, HashSet},
  fmt,
  iter::FromIterator,
};

use phf;

use crate::{state::State, StateNodeKind};

pub struct Machine {
  pub id: &'static str,
  pub root: &'static str,
  pub states: phf::OrderedMap<&'static str, StateNodeKind>,
}
impl Machine {
  pub fn state_from(&self, state_values: Vec<&str>) -> State {
    State {
      context: HashMap::new(),
      value: self.get_state_values(state_values.into_iter().map(String::from).collect(), None),
      actions: vec![],
    }
  }

  fn get_state_values(&self, state_values: Vec<String>, parent: Option<String>) -> Vec<String> {
    let parent = match parent {
      Some(p) => p,
      None => String::from(self.root),
    };

    let states: HashSet<_> = state_values
      .into_iter()
      .map(|s| {
        let potential_state = if s.starts_with(&parent) {
          String::from(&s[parent.len() + 1..])
        } else {
          String::from(s)
        };

        let index = parent.clone() + "." + &potential_state;
        if let Some(_) = self.states.get(&index) {
          return potential_state;
        } else {
          panic!("State node {} is missing", potential_state);
        }
      })
      .collect();

    Vec::from_iter(states)
  }
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let states = &self.states;
    let states_vec = states.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.root)
      .field("states", &states_vec)
      // .field("actions", &self.actions.len())
      .finish()
  }
}
