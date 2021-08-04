use std::fmt;

use crate::{action::Action, event::Event, state_node::StateNode};

pub struct TransitionConfig {
  target: Vec<String>,
}

#[derive(Debug)]
enum Kind {
  External,
  Internal,
}

pub struct Transition {
  event: String,
  source: Box<StateNode>,
  // The actual type is: String | StateNode | TransitionConfig
  config: TransitionConfig,
  actions: Vec<Action>,
  cond: Option<Box<dyn Fn(/* context type */ Event) -> bool>>,
  order: i32,
  kind: Kind,
}
impl fmt::Debug for Transition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Transition")
      .field("event", &self.event)
      .field("source", &self.source.id)
      .field(
        "target",
        &self
          .target()
          .into_iter()
          .map(|s| s.id)
          .collect::<Vec<String>>(),
      )
      // This can't actually be displayed?
      // .field("cond", &self.cond)
      .field("actions", &self.actions)
      .field("kind", &self.kind)
      .field("order", &self.order)
      .finish()
  }
}
impl Transition {
  pub fn new() -> Self {
    Transition {
      event: String::from(""),
      source: Box::new(StateNode::new()),
      config: TransitionConfig {
        target: vec![String::from("")],
      },
      actions: vec![],
      cond: None,
      order: 0,
      kind: Kind::External,
    }
  }
  pub fn target(&self) -> Vec<StateNode> {
    vec![]
  }
}
