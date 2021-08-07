use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{action::Action, state::State, state_node, state_node::StateNode};

#[derive(Debug)]
pub struct MachineConfig<'mc> {
  pub id: &'mc str,
  pub states: Vec<(&'mc str, &'mc str)>,
}

pub struct Machine {
  id: String,
  root: String,
  states: HashMap<String, StateNode>,
  // actions: Vec<Box<dyn Fn()>>,
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.root)
      // .field("config", &self.config)
      .field("states", &self.states)
      // .field("actions", &self.actions.len())
      .finish()
  }
}
impl Machine {
  // DELTE BELOW ONCE DONE
  pub fn stub() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: String::from(""),
      root: String::from(""),
      states: HashMap::new(),
    }))
  }
  // DELETE ABOVE ONCE DONE
  pub fn new(config: MachineConfig) -> Rc<RefCell<Self>> {
    let machine = Rc::new(RefCell::new(Self {
      id: String::from(config.id),
      root: String::from(config.id),
      states: HashMap::new(),
      // actions: vec![],
    }));

    let state_map = states_vec_to_map(Rc::clone(&machine), config.states, None);

    machine.borrow_mut().states = state_map;

    machine
  }
  pub fn transition(&self, state: State, event: &str) -> State {
    State::new()
  }
  pub fn state_from(&self, state_value: &str) -> State {
    State::new()
  }
  pub fn initial_state(&self) {}

  // Internal
  fn get_actions(&self, actions: Vec<Action>) -> (Vec<Box<dyn Fn()>>, Vec<String>) {
    (vec![Box::new(|| {})], vec![])
  }
  fn get_by_id(&self, id: &str) -> Option<&StateNode> {
    self.states.get(id)
  }
  fn get_configuration(&self, state_value: &str, parent: Option<&StateNode>) -> Vec<StateNode> {
    vec![StateNode::new()]
  }
}

fn states_vec_to_map(
  machine: Rc<RefCell<Machine>>,
  states: Vec<(&str, &str)>,
  parent: Option<String>,
) -> HashMap<String, StateNode> {
  states
    .into_iter()
    .fold(HashMap::new(), |mut acc, (id, _to)| {
      let current_id = match parent.clone() {
        Some(parent_id) => parent_id + "." + id,
        None => String::from(id),
      };

      acc.insert(
        current_id.clone(),
        StateNode {
          id: current_id.clone(),
          key: String::from(id),
          machine: Rc::clone(&machine),
          parent: parent.clone(),
          kind: state_node::Kind::Atomic,
          on: HashMap::new(),
          initial: None,
          entry: vec![],
          exit: vec![],
          done_data: None,
          transitions: vec![],
          states: HashMap::new(),
        },
      );

      // acc.extend(state_vec_to_map(s.states, Some(String::from(s.id))));

      acc
    })
}
