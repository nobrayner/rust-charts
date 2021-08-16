use std::{
  collections::{HashMap, HashSet},
  fmt,
  iter::FromIterator,
};

use crate::{
  action::Action,
  algorithm::enter_states,
  state::State,
  state_node::StateNode,
  state_node::{self, StateNodeConfig},
};

#[derive(Debug)]
pub struct MachineConfig<'mc> {
  pub id: &'mc str,
  pub states: Vec<StateNodeConfig<'mc>>,
}

pub struct Machine {
  id: String,
  root: String,
  pub states: HashMap<String, StateNode>,
  actions: Vec<Box<dyn Fn()>>,
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let states = &self.states;
    let states_vec = states
      .into_iter()
      .map(|(_, v)| v)
      .collect::<Vec<&StateNode>>();

    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.root)
      .field("states", &states_vec)
      // .field("actions", &self.actions.len())
      .finish()
  }
}
impl Machine {
  pub fn new(config: MachineConfig) -> Self {
    let mut machine = Self {
      id: String::from(config.id),
      root: String::from(config.id),
      states: HashMap::new(),
      actions: vec![],
    };
    // Root node
    let root_states = &config.states;
    machine.states.insert(
      String::from(config.id),
      StateNode {
        id: String::from(config.id),
        key: String::from(config.id),
        parent: None,
        kind: state_node::Kind::Atomic,
        on: HashMap::new(),
        entry: vec![],
        exit: vec![],
        done_data: None,
        transitions: vec![], // .sort_by(|a, b| a.order.cmp(&b.order)),
        states: root_states
          .into_iter()
          .map(|s| (String::from(s.id), String::from(config.id) + s.id))
          .collect(),
      },
    );

    let state_map = states_vec_to_map(config.states, String::from(config.id));

    machine.states = state_map;

    machine
  }

  pub fn transition(&self, state: State, event: &str) -> State {
    State::stub()
  }

  pub fn state_from(&self, state_values: Vec<&str>) -> State {
    State {
      context: HashMap::new(),
      value: self.get_state_values(state_values.into_iter().map(String::from).collect(), None),
      actions: vec![],
    }
  }

  pub fn initial_state(&self) -> State {
    let (configuration, actions, internal_queue) = enter_states(
      &self.states,
      vec![self.states.get(&self.root).unwrap().initial()],
      vec![],
      vec![],
      &HashMap::new(),
      vec![],
      vec![],
    );

    // let (configuration, actions) = macrostep(&self.states, configuration, actions, internal_queue);

    let (actions, warnings) = self.get_actions(actions);
    for w in warnings {
      println!("{}", w);
    }

    State {
      value: self.get_state_values(configuration, None),
      context: HashMap::new(),
      actions,
    }
  }

  // Internal
  fn get_actions(&self, actions: Vec<Action>) -> (Vec<Action>, Vec<String>) {
    (vec![], vec![])
  }

  fn get_by_id(&self, id: &str) -> Option<&StateNode> {
    self.states.get(id)
  }

  fn get_state_values(&self, state_values: Vec<String>, parent: Option<String>) -> Vec<String> {
    let parent = match parent {
      Some(p) => p,
      None => self.root.clone(),
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

fn states_vec_to_map(states: Vec<StateNodeConfig>, parent: String) -> HashMap<String, StateNode> {
  states.into_iter().fold(HashMap::new(), |mut acc, s| {
    let current_id = parent.clone() + "." + s.id;

    let child_states = &s.states;

    acc.insert(
      current_id.clone(),
      StateNode {
        id: current_id.clone(),
        key: String::from(s.id),
        parent: Some(parent.clone()),
        kind: state_node::Kind::Atomic,
        on: HashMap::new(),
        // Convert config/input into a list of actions
        entry: vec![],
        // Convert config/input into a list of actions
        exit: vec![],
        done_data: None,
        transitions: vec![], // .sort_by(|a, b| a.order.cmp(&b.order)),
        states: child_states
          .into_iter()
          .map(|s| (String::from(s.id), current_id.clone() + s.id))
          .collect(),
      },
    );

    acc.extend(states_vec_to_map(s.states, current_id));

    acc
  })
}
