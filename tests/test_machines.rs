use rust_charts::Machine;

pub static SIMPLE_LIGHTS: Machine = {
  use rust_charts::*;

  let scxml_root = State::Root(RootStateNode {});
  let green = State::Atomic(AtomicStateNode {
    id: "green",
    parent: SCXML_ROOT_ID,
    always: &[],
    on: map! {
      "TIMER" => &[
        &Transition {
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
    parent: SCXML_ROOT_ID,
    always: &[],
    on: map! {
      "TIMER" => &[
        &Transition {
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
  let red = State::Compound(CompoundStateNode {
    id: "red",
    parent: SCXML_ROOT_ID,
    always: &[],
    on: map! {
      "TIMER" => &[
        &Transition {
          targets: &["green"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red",
        },
      ],
      "done.state.red" => &[
        &Transition {
          targets: &["green"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red",
        },
      ],
    },
    initial: Some(&Transition {
      targets: &["red.walk"],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "red",
    }),
    states: &["red.walk", "red.wait", "red.stop", "red.timeout"],
    entry: &[],
    exit: &[],
  });
  let red_walk = State::Atomic(AtomicStateNode {
    id: "red.walk",
    parent: "red",
    always: &[],
    on: map! {
      "COUNTDOWN" => &[
        &Transition {
          targets: &["red.wait"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red.walk",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let red_wait = State::Atomic(AtomicStateNode {
    id: "red.wait",
    parent: "red",
    always: &[],
    on: map! {
      "COUNTDOWN" => &[
        &Transition {
          targets: &["red.stop"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red.wait",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let red_stop = State::Atomic(AtomicStateNode {
    id: "red.stop",
    parent: "red",
    always: &[],
    on: map! {
      "TIMEOUT" => &[
        &Transition {
          targets: &["red.timeout"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "red.stop",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let red_timeout = State::Final(FinalStateNode {
    id: "red.timeout",
    parent: "red",
    entry: &[],
    exit: &[],
  });

  Machine {
    id: "simple_lights",
    initial: Transition {
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
      "red.walk" => red_walk,
      "red.wait" => red_wait,
      "red.stop" => red_stop,
      "red.timeout" => red_timeout,
    },
  }
};
