use phf::OrderedMap;
use std::{fmt, ops::Deref};

use crate::{action::Action, transition::Transition};

pub trait StateNode {
  fn id(&self) -> &'static str;
  fn initial(&self) -> Option<&'static Transition>;
  fn parent(&self) -> Option<&'static str>;
  fn child_state_ids(&self) -> &[&'static str];
  /// All the transitions for this state node that are associated with an event
  fn transitions(&self) -> Vec<&'static Transition>;
  /// All the transitions for this state node that aren't associated with any events
  // fn eventless_transitions(&self);
  fn entry_actions(&self) -> Vec<&'static Action>;
  fn exit_actions(&self) -> Vec<&'static Action>;
}

pub enum State {
  Atomic(AtomicStateNode),
  Compound(CompoundStateNode),
  Final(&'static str),
  Parallel(&'static str),
  History(&'static str),
}
impl Deref for State {
  type Target = dyn StateNode;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Atomic(s) => s,
      Self::Compound(s) => s,
      _ => panic!("Not implemented yet!"),
    }
  }
}
impl fmt::Debug for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Atomic(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Compound(s) => write!(f, "<StateNode \"{}\">", &s.id),
      _ => panic!("Not implemented yet!"),
    }
  }
}

pub struct AtomicStateNode {
  pub id: &'static str,
  pub parent: Option<&'static str>,
  pub on: OrderedMap<&'static str, &'static [Transition]>,
}
impl StateNode for AtomicStateNode {
  fn id(&self) -> &'static str {
    self.id
  }
  fn parent(&self) -> Option<&'static str> {
    self.parent
  }
  fn initial(&self) -> Option<&'static Transition> {
    None
  }
  fn child_state_ids(&self) -> &'static [&'static str] {
    &[]
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    let values = self.on.values();

    values.flat_map(|v| *v).collect()
  }
  fn entry_actions(&self) -> Vec<&'static Action> {
    // TODO:
    vec![]
  }
  fn exit_actions(&self) -> Vec<&'static Action> {
    // TODO:
    vec![]
  }
}

pub struct CompoundStateNode {
  pub id: &'static str,
  pub parent: Option<&'static str>,
  pub on: OrderedMap<&'static str, &'static [Transition]>,
  pub initial: Option<&'static Transition>,
  pub states: &'static [&'static str],
}
impl StateNode for CompoundStateNode {
  fn id(&self) -> &'static str {
    self.id
  }
  fn parent(&self) -> Option<&'static str> {
    self.parent
  }
  fn initial(&self) -> Option<&'static Transition> {
    self.initial
  }
  fn child_state_ids(&self) -> &'static [&'static str] {
    self.states
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    let values = self.on.values();

    values.flat_map(|v| *v).collect()
  }
  fn entry_actions(&self) -> Vec<&'static Action> {
    // TODO:
    vec![]
  }
  fn exit_actions(&self) -> Vec<&'static Action> {
    // TODO:
    vec![]
  }
}
