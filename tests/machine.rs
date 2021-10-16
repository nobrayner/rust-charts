mod test_machines;

use std::collections::HashMap;

use test_machines::{FAN, PARALLEL, SIMPLE_LIGHTS};

#[test]
pub fn initial_state() {
  let initial_state = SIMPLE_LIGHTS.initial_state();

  assert_eq!(initial_state.configuration, vec!["green"]);
}

#[test]
pub fn transition() {
  let state = rust_charts::State {
    configuration: vec!["green".to_string()],
    actions: vec![],
    history: HashMap::new(),
  };

  let yellow_state = SIMPLE_LIGHTS.transition(state, "TIMER");

  assert_eq!(yellow_state.configuration, vec!["yellow"]);
}

#[test]
pub fn final_state() {
  let mut state = rust_charts::State {
    configuration: vec!["red".to_string(), "red.stop".to_string()],
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

#[test]
pub fn parallel_state() {
  let mut state = PARALLEL.initial_state();
  assert_eq!(
    state.configuration,
    vec![
      "steps",
      "steps.one",
      "steps.one.start",
      "steps.two",
      "steps.two.start",
    ]
  );

  state = PARALLEL.transition(state, "ONE_DONE");
  println!("{:?}", state.configuration);
  state = PARALLEL.transition(state, "TWO_DONE");
  println!("{:?}", state.configuration);
  assert_eq!(state.configuration, vec!["complete"]);
}

// // #[test]
// // pub fn state_from() {
// //   let yellow_state = SIMPLE_LIGHTS.state_from(vec!["yellow"]);

// //   assert_eq!(yellow_state.configuration, vec!["yellow"]);
// // }
