use rust_charts::*;

static SIMPLE_LIGHTS: Machine = {
  let green = State::Atomic(AtomicStateNode {
    id: "green",
    parent: None,
    on: phf_ordered_map! {
      "TIMER" => &[
        Transition {
          targets: &["yellow"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "green"
        },
      ],
    },
  });
  let yellow = State::Atomic(AtomicStateNode {
    id: "yellow",
    parent: None,
    on: phf_ordered_map! {
      "TIMER" => &[
        Transition {
          targets: &["red"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "yellow",
        },
      ],
    },
  });
  let red = State::Atomic(AtomicStateNode {
    id: "red",
    parent: None,
    on: phf_ordered_map! {
      "TIMER" => &[
        Transition {
          targets: &["green"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red",
        },
      ],
    },
  });

  Machine {
    id: "simple_lights",
    initial: &Transition {
      targets: &["green"],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "",
    },
    states: phf_ordered_map! {
      "green" => green,
      "yellow" => yellow,
      "red" => red,
    },
  }
};

#[test]
pub fn machine_initial_state() {
  let initial_state = SIMPLE_LIGHTS.initial_state();

  assert_eq!(initial_state.value, vec!["green"]);
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
