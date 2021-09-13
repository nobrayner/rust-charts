mod test_machines;

use test_machines::SIMPLE_LIGHTS;

#[test]
pub fn machine_initial_state() {
  let initial_state = SIMPLE_LIGHTS.initial_state();

  assert_eq!(initial_state.configuration, vec!["green"]);
}

#[test]
pub fn machine() {
  let mut state = SIMPLE_LIGHTS.initial_state();
  state = SIMPLE_LIGHTS.transition(state, "TIMER");
  state = SIMPLE_LIGHTS.transition(state, "TIMER");

  assert_eq!(state.configuration, vec!["red", "red.walk"]);

  state = SIMPLE_LIGHTS.transition(state, "COUNTDOWN");
  state = SIMPLE_LIGHTS.transition(state, "COUNTDOWN");
  state = SIMPLE_LIGHTS.transition(state, "TIMEOUT");

  assert_eq!(state.configuration, vec!["green"]);
}

// #[test]
// pub fn state_from() {
//   let yellow_state = SIMPLE_LIGHTS.state_from(vec!["yellow"]);

//   assert_eq!(yellow_state.configuration, vec!["yellow"]);
// }
