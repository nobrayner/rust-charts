use std::collections::HashMap;

use crate::{
  event::Event,
  state_node::{self, StateNode},
  transition::Transition,
};

pub fn is_compound_state(state_map: &HashMap<String, StateNode>, state_id: &String) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state.kind {
      state_node::Kind::Compound => true,
      _ => false,
    }
  } else {
    false
  }
}

pub fn is_atomic_state(state_map: &HashMap<String, StateNode>, state_id: &String) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state.kind {
      state_node::Kind::Atomic => true,
      state_node::Kind::Final => true,
      state_node::Kind::History => true,
      _ => false,
    }
  } else {
    false
  }
}

pub fn is_history_state(state_map: &HashMap<String, StateNode>, state_id: &String) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state.kind {
      state_node::Kind::History => true,
      _ => false,
    }
  } else {
    false
  }
}

pub fn is_parallel_state(state_map: &HashMap<String, StateNode>, state_id: &String) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state.kind {
      state_node::Kind::Parallel => true,
      _ => false,
    }
  } else {
    false
  }
}

pub fn is_final_state(state_map: &HashMap<String, StateNode>, state_id: &String) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state.kind {
      state_node::Kind::Final => true,
      _ => false,
    }
  } else {
    false
  }
}

pub fn is_descendant(maybe_child_id: &String, maybe_parent_id: &String) -> bool {
  maybe_child_id.starts_with(maybe_parent_id)
}

pub fn get_child_states(state_map: &HashMap<String, StateNode>, state_id: &String) -> Vec<String> {
  if let Some(node) = state_map.get(state_id) {
    node.states.values().map(String::from).collect()
  } else {
    vec![]
  }
}

pub fn is_in_final_state(
  state_map: &HashMap<String, StateNode>,
  state_id: &String,
  configuration: &Vec<String>,
) -> bool {
  if is_compound_state(state_map, state_id) {
    get_child_states(state_map, state_id)
      .iter()
      .any(|s| is_final_state(state_map, s) && configuration.contains(s))
  } else if is_parallel_state(state_map, state_id) {
    get_child_states(state_map, state_id)
      .iter()
      .all(|s| is_in_final_state(state_map, s, configuration))
  } else {
    false
  }
}

pub fn condition_match(transition: &Transition) -> bool {
  match transition.cond {
    Some(cond) => cond(
      // FIXME: Use real event and context here?
      Event {
        name: String::from(""),
        data: HashMap::new(),
      },
    ),
    None => true,
  }
}
