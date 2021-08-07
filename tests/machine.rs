use std::{cell::RefCell, rc::Rc};

use xstate_rust::*;

thread_local!(static LIGHTS_MACHINE: Rc<RefCell<Machine>> = Machine::new(MachineConfig {
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
}));

// #[test]
// pub fn test_machine() {
//   let yellow_state = LIGHTS_MACHINE.transition(LIGHTS_MACHINE.initial_state(), "TIMER");

//   assert_eq!(yellow_state.value, "yellow");

//   let red_state = LIGHTS_MACHINE.transition(yellow_state, "TIMER");

//   assert_eq!(red_state.value, "red.walk");
// }

// #[test]
// pub fn test_machine_initial_state() {
//   assert_eq!(LIGHTS_MACHINE.initial_state().value, "green");
// }

// #[test]
// pub fn test_final_state() {
//   let red_stop_state = LIGHTS_MACHINE.state_from("red.stop");

//   let red_timeout_state = lights.transition(red_stop_state, "TIMEOUT");

//   assert_eq!(red_timeout_state.value, "green");
// }
