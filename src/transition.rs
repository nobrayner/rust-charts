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
  /*
    I don't think that event will be needed, as a state can just have the different types of events in their own lists:
    - always (Eventless transitions)
    - on (Event transitions (Event can be determined by the key of the map))
    - onDone (done.state.*, done.invoke.* Events)
    - onError (error.* Events)
  */
  // pub(crate) event: String,
  // pub(crate) order: i32,
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
      // .field("guard", &self.guard)
      .field("kind", &self.kind)
      .field("source", &self.source)
      // .field("event", &self.event)
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
      // event: String::from(""),
      // order: 0,
    }
  }
}
