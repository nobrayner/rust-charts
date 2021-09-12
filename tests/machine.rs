use rust_charts::Machine;

static SIMPLE_LIGHTS: Machine = {
  use rust_charts::*;

  let scxml_root = State::Root(RootStateNode {});
  let green = State::Atomic(AtomicStateNode {
    id: "green",
    parent: Some("scxml::root"),
    always: &[],
    on: map! {
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
    entry: &[],
    exit: &[],
  });
  let yellow = State::Atomic(AtomicStateNode {
    id: "yellow",
    parent: Some("scxml::root"),
    always: &[],
    on: map! {
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
    entry: &[],
    exit: &[],
  });
  let red = State::Atomic(AtomicStateNode {
    id: "red",
    parent: Some("scxml::root"),
    always: &[],
    on: map! {
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
    entry: &[],
    exit: &[],
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
    states: map! {
      // TODO: Use the const (crate::state_node::SCXML_ROOT_ID) somehow
      "scxml::root" => scxml_root,
      "green" => green,
      "yellow" => yellow,
      "red" => red,
    },
  }
};

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
