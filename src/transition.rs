use std::fmt;

use crate::{action::Action, event::Event};

#[derive(Clone)]
pub struct TransitionConfig {
  target: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum Kind {
  External,
  Internal,
}

#[derive(Clone)]
pub struct Transition {
  pub(crate) event: String,
  pub(crate) source: &'static str,
  // The actual type is: String | StateNode | TransitionConfig
  config: TransitionConfig,
  actions: Vec<Action>,
  pub(crate) cond: Option<fn(/* context type */ Event) -> bool>,
  pub(crate) order: i32,
  pub(crate) kind: Kind,
}
impl fmt::Debug for Transition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Transition")
      .field("event", &self.event)
      .field("source", &self.source)
      .field("target", &self.target())
      // This can't actually be displayed?
      // .field("cond", &self.cond)
      .field("actions", &self.actions)
      .field("kind", &self.kind)
      .field("order", &self.order)
      .finish()
  }
}
impl Transition {
  pub fn stub() -> Self {
    Transition {
      event: String::from(""),
      source: "",
      config: TransitionConfig {
        target: vec![String::from("")],
      },
      actions: vec![],
      cond: None,
      order: 0,
      kind: Kind::External,
    }
  }
  pub fn target(&self) -> Vec<&'static str> {
    vec![]
  }
}
