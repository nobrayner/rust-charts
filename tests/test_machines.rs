use rust_charts::Machine;

pub static SIMPLE_LIGHTS: Machine = {
  use rust_charts::*;

  let scxml_root = StateNode::Root(RootStateNode {});
  let green = StateNode::Atomic(AtomicStateNode {
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
  let yellow = StateNode::Atomic(AtomicStateNode {
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
  let red = StateNode::Compound(CompoundStateNode {
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
    history_states: &[],
    entry: &[],
    exit: &[],
  });
  let red_walk = StateNode::Atomic(AtomicStateNode {
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
  let red_wait = StateNode::Atomic(AtomicStateNode {
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
  let red_stop = StateNode::Atomic(AtomicStateNode {
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
  let red_timeout = StateNode::Final(FinalStateNode {
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

pub static FAN: Machine = {
  use rust_charts::*;

  let scxml_root = StateNode::Root(RootStateNode {});

  let off = StateNode::Atomic(AtomicStateNode {
    id: "off",
    parent: SCXML_ROOT_ID,
    always: &[],
    on: map! {
      "POWER" => &[
        &Transition {
          targets: &["on.hist"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "off",
        },
      ],
      "HIGH_POWER" => &[
        &Transition {
          targets: &["on.third"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "off",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let on = StateNode::Compound(CompoundStateNode {
    id: "on",
    parent: SCXML_ROOT_ID,
    always: &[],
    on: map! {
      "POWER" => &[
        &Transition {
          targets: &["off"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "on",
        },
      ]
    },
    initial: Some(&Transition {
      targets: &["on.first"],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "on",
    }),
    states: &["on.first", "on.second", "on.third"],
    history_states: &["on.hist"],
    entry: &[],
    exit: &[],
  });
  let on_first = StateNode::Atomic(AtomicStateNode {
    id: "on.first",
    parent: "on",
    always: &[],
    on: map! {
      "SWITCH" => &[
        &Transition {
          targets: &["on.second"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "on.first",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let on_second = StateNode::Atomic(AtomicStateNode {
    id: "on.second",
    parent: "on",
    always: &[],
    on: map! {
      "SWITCH" => &[
        &Transition {
          targets: &["on.third"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "on.second",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let on_third = StateNode::Atomic(AtomicStateNode {
    id: "on.third",
    parent: "on",
    always: &[],
    on: map! {
      "SWITCH" => &[
        &Transition {
          targets: &["on.first"],
          actions: &[],
          guard: None,
          kind: TransitionKind::External,
          source: "on.third",
        },
      ],
    },
    entry: &[],
    exit: &[],
  });
  let on_hist = StateNode::History(HistoryStateNode {
    id: "on.hist",
    parent: "on",
    kind: HistoryKind::Shallow,
    transition: &Transition {
      targets: &["on.first"],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "on.hist",
    },
  });

  Machine {
    id: "fan",
    initial: Transition {
      targets: &["off"],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "",
    },
    states: map! {
      "scxml::root" => scxml_root,
      "off" => off,
      "on" => on,
      "on.first" => on_first,
      "on.second" => on_second,
      "on.third" => on_third,
      "on.hist" => on_hist,
    },
  }
};
