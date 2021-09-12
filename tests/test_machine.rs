use rust_charts::Machine;

pub static SIMPLE_LIGHTS: Machine = {
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
    },
  }
};
