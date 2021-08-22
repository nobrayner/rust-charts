use rust_charts::*;

static SIMPLE_LIGHTS: Machine = {
  let root_transitions = phf_ordered_map! {};
  let green_transitions = phf_ordered_map! {
    "TIMER" => "yellow",
  };
  let yellow_transitions = phf_ordered_map! {
    "TIMER" => "red",
  };
  let red_transitions = phf_ordered_map! {
    "TIMER" => "green",
  };

  let red_states = phf_ordered_map! {
    "green" => "root.green",
    "yellow" => "root.yellow",
    "red" => "root.red",
  };

  Machine {
    id: "simple_lights",
    root: "root",
    states: phf_ordered_map! {
      "root" => State::Compound(CompoundStateNode {
        id: "root",
        key: "root",
        parent: None,
        on: root_transitions,
        initial: "root.green",
        states: red_states,
      }),
      "root.green" => State::Atomic(AtomicStateNode {
        id: "root.green",
        key: "green",
        parent: Some("root"),
        on: green_transitions,
      }),
      "root.yellow" => State::Atomic(AtomicStateNode {
        id: "root.yellow",
        key: "yellow",
        parent: Some("root"),
        on: yellow_transitions,
      }),
      "root.red" => State::Atomic(AtomicStateNode {
        id: "root.red",
        key: "red",
        parent: Some("root"),
        on: red_transitions,
      }),
    },
  }
};

// #[test]
// pub fn machine_initial_state() {
//   let mut lights = light_machine();

//   assert_eq!(lights.initial_state().value, vec!["green"]);
// }

#[test]
pub fn state_from() {
  let yellow_state = SIMPLE_LIGHTS.state_from(vec!["yellow"]);

  println!("{:?}", SIMPLE_LIGHTS);

  assert_eq!(yellow_state.value, vec!["yellow"]);
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
