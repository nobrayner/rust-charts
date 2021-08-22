use std::{
  collections::{HashMap, HashSet},
  fmt,
  iter::FromIterator,
};

use phf;

use crate::{algorithm, state::State, state_node, state_node::StateNode};

pub struct Machine {
  pub id: &'static str,
  pub root: &'static str,
  pub states: phf::OrderedMap<&'static str, state_node::State>,
}
impl Machine {
  pub fn state_from(&self, state_values: Vec<&'static str>) -> State {
    State {
      context: HashMap::new(),
      value: self.get_state_values(state_values, None),
      actions: vec![],
    }
  }

  pub fn initial_state(&self) -> State {
    let (configuration, actions, internal_queue) = algorithm::enter_states(
      &self.states,
      vec![self.states.get(&self.root).unwrap().initial()],
      vec![],
      vec![],
      &HashMap::new(),
      vec![],
      vec![],
    );

    // let (configuration, actions) = macrostep(&self.states, configuration, actions, internal_queue);

    // let (actions, warnings) = self.get_actions(actions);
    // for w in warnings {
    //   println!("{}", w);
    // }

    State {
      value: self.get_state_values(configuration, None),
      context: HashMap::new(),
      actions,
    }
  }

  fn get_state_values(
    &self,
    state_values: Vec<&'static str>,
    parent: Option<&'static str>,
  ) -> Vec<&'static str> {
    let parent = match parent {
      Some(p) => p,
      None => self.root,
    };

    let states: HashSet<_> = state_values
      .into_iter()
      .map(|s| {
        let potential_state = if s.starts_with(parent) {
          &s[parent.len() + 1..]
        } else {
          s
        };

        let index = String::from(parent) + "." + &potential_state;
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
