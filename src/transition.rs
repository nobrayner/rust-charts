use std::fmt;

use crate::{action::Action, event::Event};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TransitionKind {
  External,
  Internal,
}

pub struct TransitionConfig {
  pub targets: &'static [&'static str],
  pub cond: Option<fn(/* context type */ Event) -> bool>,
  pub kind: TransitionKind,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Transition {
  pub(crate) event: String,
  pub(crate) source: &'static str,
  pub(crate) targets: Vec<&'static str>,
  pub(crate) actions: Vec<Action>,
  pub(crate) cond: Option<fn(/* context type */ Event) -> bool>,
  pub(crate) order: i32,
  pub(crate) kind: TransitionKind,
}
impl fmt::Debug for Transition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Transition")
      .field("event", &self.event)
      .field("source", &self.source)
      .field("target", &self.targets)
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
      targets: vec![""],
      actions: vec![],
      cond: None,
      order: 0,
      kind: TransitionKind::External,
    }
  }
}
