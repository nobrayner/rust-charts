use phf::OrderedMap;
use std::{fmt, ops::Deref};

use crate::{action::Action, transition::Transition};

pub const SCXML_ROOT_ID: &str = "scxml::root";

pub trait StateNode {
  fn id(&self) -> &'static str;
  fn initial(&self) -> Option<&'static Transition>;
  fn parent(&self) -> Option<&'static str>;
  fn child_state_ids(&self) -> &[&'static str];
  /// Transitions for this state node that aren't associated with any events (always transitions)
  fn eventless_transitions(&self) -> Vec<&'static Transition>;
  /// Transitions for this state node that are associated with an event
  fn transitions(&self) -> Vec<&'static Transition>;
  /// Transitions associated with the provided event name
  fn on(&self, event_name: &str) -> Vec<&'static Transition>;
  fn entry_actions(&self) -> Vec<&'static Action>;
  fn exit_actions(&self) -> Vec<&'static Action>;
}

pub enum State {
  Root(RootStateNode),
  Atomic(AtomicStateNode),
  Compound(CompoundStateNode),
  Final(FinalStateNode),
  Parallel(&'static str),
  History(&'static str),
}
impl Deref for State {
  type Target = dyn StateNode;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Root(s) => s,
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

pub struct RootStateNode {}
impl StateNode for RootStateNode {
  fn id(&self) -> &'static str {
    SCXML_ROOT_ID
  }
  fn parent(&self) -> Option<&'static str> {
    None
  }
  fn initial(&self) -> Option<&'static Transition> {
    None
  }
  fn child_state_ids(&self) -> &[&'static str] {
    &[]
  }
  fn eventless_transitions(&self) -> Vec<&'static Transition> {
    vec![]
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    vec![]
  }
  fn on(&self, _: &str) -> Vec<&'static Transition> {
    vec![]
  }
  fn entry_actions(&self) -> Vec<&'static Action> {
    vec![]
  }
  fn exit_actions(&self) -> Vec<&'static Action> {
    vec![]
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
  fn eventless_transitions(&self) -> Vec<&'static Transition> {
    // TODO: always property
    vec![]
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    let values = self.on.values();

    values.flat_map(|v| *v).collect()
  }
  fn on(&self, event_name: &str) -> Vec<&'static Transition> {
    match self.on.get(event_name) {
      Some(&transitions) => transitions.iter().collect(),
      None => vec![],
    }
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
  fn eventless_transitions(&self) -> Vec<&'static Transition> {
    // TODO: always property
    vec![]
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    let values = self.on.values();

    values.flat_map(|v| *v).collect()
  }
  fn on(&self, event_name: &str) -> Vec<&'static Transition> {
    match self.on.get(event_name) {
      Some(&transitions) => transitions.iter().collect(),
      None => vec![],
    }
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

pub struct FinalStateNode {
  pub id: &'static str,
  pub parent: Option<&'static str>,
}
impl StateNode for FinalStateNode {
  fn id(&self) -> &'static str {
    self.id
  }
  fn initial(&self) -> Option<&'static Transition> {
    None
  }
  fn parent(&self) -> Option<&'static str> {
    self.parent
  }
  fn child_state_ids(&self) -> &[&'static str] {
    &[]
  }
  fn eventless_transitions(&self) -> Vec<&'static Transition> {
    vec![]
  }
  fn transitions(&self) -> Vec<&'static Transition> {
    vec![]
  }
  fn on(&self, _: &str) -> Vec<&'static Transition> {
    vec![]
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
