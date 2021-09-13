use std::fmt;

use crate::{action::Action, event::Event};

#[derive(Debug, PartialEq)]
pub enum TransitionKind {
  External,
  Internal,
}

pub struct Transition {
  pub targets: &'static [&'static str],
  pub actions: &'static [&'static Action],
  pub guard: Option<fn(/* context type */ &Event) -> bool>,
  pub kind: TransitionKind,
  pub source: &'static str,
}
impl PartialEq for Transition {
  fn eq(&self, other: &Self) -> bool {
    self.targets == other.targets
      && self.actions == other.actions
      && self.kind == other.kind
      && self.source == other.source
  }
}
impl fmt::Debug for Transition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Transition")
      .field("targets", &self.targets)
      .field("actions", &self.actions)
      // This can't actually be displayed?
      .field(
        "guard",
        match &self.guard {
          Some(_) => &"fn (Event) -> bool",
          None => &"None",
        },
      )
      .field("kind", &self.kind)
      .field("source", &self.source)
      .finish()
  }
}
impl Transition {
  pub fn stub() -> Self {
    Transition {
      targets: &[],
      actions: &[],
      guard: None,
      kind: TransitionKind::External,
      source: "",
    }
  }
}
