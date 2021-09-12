mod test_machine;

use test_machine::SIMPLE_LIGHTS;

#[test]
pub fn machine_initial_state() {
  let initial_state = SIMPLE_LIGHTS.initial_state();

  assert_eq!(initial_state.configuration, vec!["green"]);
}

#[test]
pub fn machine() {
  let yellow_state = SIMPLE_LIGHTS.transition(SIMPLE_LIGHTS.initial_state(), "TIMER");

  assert_eq!(yellow_state.configuration, vec!["yellow"]);

  let red_state = SIMPLE_LIGHTS.transition(yellow_state, "TIMER");

  assert_eq!(red_state.configuration, vec!["red"]);
}

// #[test]
// pub fn final_state() {
//   let lights = light_machine();

//   let red_stop_state = lights.state_from(vec!["red.stop"]);

//   let red_timeout_state = lights.transition(red_stop_state, "TIMEOUT");

//   assert_eq!(red_timeout_state.value, vec![String::from("green")]);
// }

// #[test]
// pub fn machine() {
//   let lights = light_machine();

//   let yellow_state = lights.transition(lights.initial_state(), "TIMER");

//   assert_eq!(yellow_state.value, "yellow");

//   let red_state = lights.transition(yellow_state, "TIMER");

//   assert_eq!(red_state.value, "red.walk");
// }

// #[test]
// pub fn state_from() {
//   let yellow_state = SIMPLE_LIGHTS.state_from(vec!["yellow"]);

//   assert_eq!(yellow_state.value, vec!["yellow"]);
// }
