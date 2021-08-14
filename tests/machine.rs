use xstate_rust::*;

fn light_machine() -> Machine {
  Machine::new(MachineConfig {
    id: "lights",
    states: vec![
      StateNodeConfig {
        id: "green",
        on: vec![("TIMER", "yellow")],
        kind: StateNodeKind::Atomic,
        initial: None,
        on_done: None,
        states: vec![],
      },
      StateNodeConfig {
        id: "yellow",
        on: vec![("TIMER", "red")],
        kind: StateNodeKind::Atomic,
        initial: None,
        on_done: None,
        states: vec![],
      },
      StateNodeConfig {
        id: "red",
        on: vec![],
        kind: StateNodeKind::Atomic,
        initial: None,
        on_done: None,
        states: vec![
          StateNodeConfig {
            id: "walk",
            kind: StateNodeKind::Atomic,
            on: vec![("COUNTDOWN", "wait")],
            initial: None,
            states: vec![],
            on_done: None,
          },
          StateNodeConfig {
            id: "wait",
            kind: StateNodeKind::Atomic,
            on: vec![("COUNTDOWN", "stop")],
            initial: None,
            states: vec![],
            on_done: None,
          },
          StateNodeConfig {
            id: "stop",
            kind: StateNodeKind::Atomic,
            on: vec![("TIMEOUT", "timeout")],
            initial: None,
            states: vec![],
            on_done: None,
          },
          StateNodeConfig {
            id: "timeout",
            kind: StateNodeKind::Final,
            on: vec![],
            initial: None,
            states: vec![],
            on_done: None,
          },
        ],
      },
    ],
  })
}

// #[test]
// pub fn machine_initial_state() {
//   let mut lights = light_machine();

//   assert_eq!(lights.initial_state().value, vec!["green"]);
// }

#[test]
pub fn state_from() {
  let lights = light_machine();

  let yellow_state = lights.state_from(vec!["yellow"]);

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
