mod test_machines;

use std::collections::HashMap;

use test_machines::{FAN, SIMPLE_LIGHTS};

#[test]
pub fn machine_initial_state() {
  let initial_state = SIMPLE_LIGHTS.initial_state();

  assert_eq!(initial_state.configuration, vec!["green"]);
}

#[test]
pub fn machine_simple_transition() {
  let state = rust_charts::State {
    configuration: vec!["green"],
    actions: vec![],
    history: HashMap::new(),
  };

  let yellow_state = SIMPLE_LIGHTS.transition(state, "TIMER");

  assert_eq!(yellow_state.configuration, vec!["yellow"]);
}

#[test]
pub fn child_final_state() {
  let mut state = rust_charts::State {
    configuration: vec!["red", "red.stop"],
    actions: vec![],
    history: HashMap::new(),
  };

  state = SIMPLE_LIGHTS.transition(state, "TIMEOUT");

  assert_eq!(state.configuration, vec!["green"]);
}

#[test]
pub fn history_state() {
  let mut state = FAN.initial_state();
  state = FAN.transition(state, "POWER");
  assert_eq!(state.configuration, vec!["on.first", "on"]);

  state = FAN.transition(state, "SWITCH");
  state = FAN.transition(state, "POWER");
  assert_eq!(state.configuration, vec!["off"]);
  assert_eq!(
    format!("{:?}", state.history),
    "{\"on.hist\": [\"on.second\"]}",
    "correctly stores historical configuration"
  );

  state = FAN.transition(state, "POWER");
  assert_eq!(
    state.configuration,
    vec!["on.second", "on"],
    "correctly enters historical configuration"
  );
}

// #[test]
// pub fn state_from() {
//   let yellow_state = SIMPLE_LIGHTS.state_from(vec!["yellow"]);

//   assert_eq!(yellow_state.configuration, vec!["yellow"]);
// }
