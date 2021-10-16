use rust_charts::{lazy_static, Machine};

lazy_static! {
  pub static ref SIMPLE_LIGHTS: Machine = {
    use rust_charts::*;

    let scxml_root = StateNode::Root(RootStateNode {});
    let green = StateNode::Atomic(AtomicStateNode {
      id: "green".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "TIMER".to_string() => vec![
          Transition::new(
            vec!["yellow".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "green".to_string()
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let yellow = StateNode::Atomic(AtomicStateNode {
      id: "yellow".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "TIMER".to_string() => vec![
          Transition::new(
            vec!["red".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "yellow".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let red = StateNode::Compound(CompoundStateNode {
      id: "red".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "TIMER".to_string() => vec![
          Transition::new(
            vec!["green".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "red".to_string(),
          ),
        ],
        "done.state.red".to_string() => vec![
          Transition::new(
            vec!["green".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "red".to_string(),
          ),
        ],
      },
      initial: Transition::new(
        vec!["red.walk".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "red".to_string(),
      ),
      states: vec![
        "red.walk".to_string(),
        "red.wait".to_string(),
        "red.stop".to_string(),
        "red.timeout".to_string(),
      ],
      history_states: vec![],
      entry: vec![],
      exit: vec![],
    });
    let red_walk = StateNode::Atomic(AtomicStateNode {
      id: "red.walk".to_string(),
      parent: "red".to_string(),
      always: vec![],
      on: map! {
        "COUNTDOWN".to_string() => vec![
          Transition::new(
            vec!["red.wait".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "red.walk".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let red_wait = StateNode::Atomic(AtomicStateNode {
      id: "red.wait".to_string(),
      parent: "red".to_string(),
      always: vec![],
      on: map! {
        "COUNTDOWN".to_string() => vec![
          Transition::new(
            vec!["red.stop".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "red.wait".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let red_stop = StateNode::Atomic(AtomicStateNode {
      id: "red.stop".to_string(),
      parent: "red".to_string(),
      always: vec![],
      on: map! {
        "TIMEOUT".to_string() => vec![
          Transition::new(
            vec!["red.timeout".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "red.stop".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let red_timeout = StateNode::Final(FinalStateNode {
      id: "red.timeout".to_string(),
      parent: "red".to_string(),
      entry: vec![],
      exit: vec![],
    });

    Machine::new(
      "simple_lights",
      "green".to_string(),
      map! {
        // TODO: Use the const (crate::state_node::SCXML_ROOT_ID) somehow
        "scxml::root".to_string() => scxml_root,
        "green".to_string() => green,
        "yellow".to_string() => yellow,
        "red".to_string() => red,
        "red.walk".to_string() => red_walk,
        "red.wait".to_string() => red_wait,
        "red.stop".to_string() => red_stop,
        "red.timeout".to_string() => red_timeout,
      },
      map! {},
      map! {},
    )
  };
}

lazy_static! {
  pub static ref FAN: Machine = {
    use rust_charts::*;

    let scxml_root = StateNode::Root(RootStateNode {});

    let off = StateNode::Atomic(AtomicStateNode {
      id: "off".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "POWER".to_string() => vec![
          Transition::new(
            vec!["on.hist".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "off".to_string(),
          ),
        ],
        "HIGH_POWER".to_string() => vec![
          Transition::new(
            vec!["on.third".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "off".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let on = StateNode::Compound(CompoundStateNode {
      id: "on".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "POWER".to_string() => vec![
          Transition::new(
            vec!["off".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "on".to_string(),
          ),
        ]
      },
      initial: Transition::new(
        vec!["on.first".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "on".to_string(),
      ),
      states: vec![
        "on.first".to_string(),
        "on.second".to_string(),
        "on.third".to_string(),
      ],
      history_states: vec!["on.hist".to_string()],
      entry: vec![],
      exit: vec![],
    });
    let on_first = StateNode::Atomic(AtomicStateNode {
      id: "on.first".to_string(),
      parent: "on".to_string(),
      always: vec![],
      on: map! {
        "SWITCH".to_string() => vec![
          Transition::new(
            vec!["on.second".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "on.first".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let on_second = StateNode::Atomic(AtomicStateNode {
      id: "on.second".to_string(),
      parent: "on".to_string(),
      always: vec![],
      on: map! {
        "SWITCH".to_string() => vec![
          Transition::new(
            vec!["on.third".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "on.second".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let on_third = StateNode::Atomic(AtomicStateNode {
      id: "on.third".to_string(),
      parent: "on".to_string(),
      always: vec![],
      on: map! {
        "SWITCH".to_string() => vec![
          Transition::new(
            vec!["on.first".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "on.third".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let on_hist = StateNode::History(HistoryStateNode {
      id: "on.hist".to_string(),
      parent: "on".to_string(),
      kind: HistoryKind::Shallow,
      transition: Transition::new(
        vec!["on.first".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "on.hist".to_string(),
      ),
    });

    Machine::new(
      "fan",
      "off".to_string(),
      map! {
        "scxml::root".to_string() => scxml_root,
        "off".to_string() => off,
        "on".to_string() => on,
        "on.first".to_string() => on_first,
        "on.second".to_string() => on_second,
        "on.third".to_string() => on_third,
        "on.hist".to_string() => on_hist,
      },
      map! {},
      map! {},
    )
  };
}

lazy_static! {
  pub static ref PARALLEL: Machine = {
    use rust_charts::*;

    let scxml_root = StateNode::Root(RootStateNode {});
    let steps = StateNode::Parallel(ParallelStateNode {
      id: "steps".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {
        "done.state.steps".to_string() => vec![
          Transition::new(
            vec!["complete".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "steps".to_string(),
          ),
        ],
      },
      initial: Transition::new(
        vec!["steps.one".to_string(), "steps.two".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "steps".to_string(),
      ),
      states: vec!["steps.one".to_string(), "steps.two".to_string()],
      history_states: vec![],
      entry: vec![],
      exit: vec![],
    });
    let steps_one = StateNode::Compound(CompoundStateNode {
      id: "steps.one".to_string(),
      parent: "steps".to_string(),
      always: vec![],
      on: map! {},
      history_states: vec![],
      initial: Transition::new(
        vec!["steps.one.start".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "steps.one".to_string(),
      ),
      states: vec!["steps.one.start".to_string(), "steps.one.done".to_string()],
      entry: vec![],
      exit: vec![],
    });
    let steps_one_start = StateNode::Atomic(AtomicStateNode {
      id: "steps.one.start".to_string(),
      parent: "steps.one".to_string(),
      always: vec![],
      on: map! {
        "ONE_DONE".to_string() => vec![
          Transition::new(
            vec!["steps.one.done".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "steps.one.start".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let steps_one_done = StateNode::Final(FinalStateNode {
      id: "steps.one.done".to_string(),
      parent: "steps.one".to_string(),
      entry: vec![],
      exit: vec![],
    });
    let steps_two = StateNode::Compound(CompoundStateNode {
      id: "steps.two".to_string(),
      parent: "steps".to_string(),
      always: vec![],
      on: map! {},
      history_states: vec![],
      initial: Transition::new(
        vec!["steps.two.start".to_string()],
        vec![],
        None,
        TransitionKind::External,
        "steps.two".to_string(),
      ),
      states: vec!["steps.two.start".to_string(), "steps.two.done".to_string()],
      entry: vec![],
      exit: vec![],
    });
    let steps_two_start = StateNode::Atomic(AtomicStateNode {
      id: "steps.two.start".to_string(),
      parent: "steps.two".to_string(),
      always: vec![],
      on: map! {
        "TWO_DONE".to_string() => vec![
          Transition::new(
            vec!["steps.two.done".to_string()],
            vec![],
            None,
            TransitionKind::External,
            "steps.two.start".to_string(),
          ),
        ],
      },
      entry: vec![],
      exit: vec![],
    });
    let steps_two_done = StateNode::Final(FinalStateNode {
      id: "steps.two.done".to_string(),
      parent: "steps.two".to_string(),
      entry: vec![],
      exit: vec![],
    });
    let complete = StateNode::Atomic(AtomicStateNode {
      id: "complete".to_string(),
      parent: SCXML_ROOT_ID.to_string(),
      always: vec![],
      on: map! {},
      entry: vec![],
      exit: vec![],
    });

    Machine::new(
      "parallel",
      "steps".to_string(),
      map! {
        "scxml::root".to_string() => scxml_root,
        "steps".to_string() => steps,
        "steps.one".to_string() => steps_one,
        "steps.one.start".to_string() => steps_one_start,
        "steps.one.done".to_string() => steps_one_done,
        "steps.two".to_string() => steps_two,
        "steps.two.start".to_string() => steps_two_start,
        "steps.two.done".to_string() => steps_two_done,
        "complete".to_string() => complete,
      },
      map! {},
      map! {},
    )
  };
}
