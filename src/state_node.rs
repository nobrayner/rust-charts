use std::{collections::HashMap, fmt, ops::Deref};

use crate::transition::Transition;

pub const SCXML_ROOT_ID: &str = "scxml::root";

// TODO: onDone and onError transitions

pub trait StateNodeTrait {
  fn id(&self) -> &str;
  fn initial(&self) -> Option<&Transition>;
  fn parent(&self) -> Option<&String>;
  fn child_state_ids(&self) -> &[String];
  /// Transitions for this state node that aren't associated with any events (always transitions)
  fn eventless_transitions(&self) -> &[Transition];
  /// Transitions associated with the provided event name (includes done.* and error.* events)
  fn on(&self, event_name: &str) -> &[Transition];
  fn history_state_ids(&self) -> &[String];
  fn entry_actions(&self) -> &[String];
  fn exit_actions(&self) -> &[String];
}

pub enum StateNode {
  Root(RootStateNode),
  Atomic(AtomicStateNode),
  Compound(CompoundStateNode),
  Final(FinalStateNode),
  Parallel(ParallelStateNode),
  History(HistoryStateNode),
}
impl Deref for StateNode {
  type Target = dyn StateNodeTrait;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Root(s) => s,
      Self::Atomic(s) => s,
      Self::Compound(s) => s,
      Self::Final(s) => s,
      Self::Parallel(s) => s,
      Self::History(s) => s,
    }
  }
}
impl fmt::Debug for StateNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Root(_) => write!(f, "<StateNode \"{}\">", SCXML_ROOT_ID),
      Self::Atomic(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Compound(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Final(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Parallel(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::History(s) => write!(f, "<StateNode \"{}\">", &s.id),
    }
  }
}

pub struct RootStateNode {}
impl StateNodeTrait for RootStateNode {
  fn id(&self) -> &str {
    SCXML_ROOT_ID
  }
  fn parent(&self) -> Option<&String> {
    None
  }
  fn initial(&self) -> Option<&Transition> {
    None
  }
  fn child_state_ids(&self) -> &[String] {
    &[]
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &[]
  }
  fn on(&self, _: &str) -> &[Transition] {
    &[]
  }
  fn history_state_ids(&self) -> &[String] {
    &[]
  }
  fn entry_actions(&self) -> &[String] {
    &[]
  }
  fn exit_actions(&self) -> &[String] {
    &[]
  }
}

pub struct AtomicStateNode {
  pub id: String,
  pub parent: String,
  pub always: Vec<Transition>,
  pub on: HashMap<String, Vec<Transition>>,
  pub entry: Vec<String>,
  pub exit: Vec<String>,
}
impl StateNodeTrait for AtomicStateNode {
  fn id(&self) -> &str {
    &self.id
  }
  fn parent(&self) -> Option<&String> {
    Some(&self.parent)
  }
  fn initial(&self) -> Option<&Transition> {
    None
  }
  fn child_state_ids(&self) -> &[String] {
    &[]
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &self.always
  }
  fn on(&self, event_name: &str) -> &[Transition] {
    match self.on.get(event_name) {
      Some(transitions) => transitions,
      None => &[],
    }
  }
  fn history_state_ids(&self) -> &[String] {
    &[]
  }
  fn entry_actions(&self) -> &[String] {
    &self.entry
  }
  fn exit_actions(&self) -> &[String] {
    &self.exit
  }
}

pub struct CompoundStateNode {
  pub id: String,
  pub parent: String,
  pub always: Vec<Transition>,
  pub on: HashMap<String, Vec<Transition>>,
  pub initial: Transition,
  pub states: Vec<String>,
  pub history_states: Vec<String>,
  pub entry: Vec<String>,
  pub exit: Vec<String>,
}
impl StateNodeTrait for CompoundStateNode {
  fn id(&self) -> &str {
    &self.id
  }
  fn parent(&self) -> Option<&String> {
    Some(&self.parent)
  }
  fn initial(&self) -> Option<&Transition> {
    Some(&self.initial)
  }
  fn child_state_ids(&self) -> &[String] {
    &self.states
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &self.always
  }
  fn on(&self, event_name: &str) -> &[Transition] {
    match self.on.get(event_name) {
      Some(transitions) => transitions,
      None => &[],
    }
  }
  fn history_state_ids(&self) -> &[String] {
    &self.history_states
  }
  fn entry_actions(&self) -> &[String] {
    &self.entry
  }
  fn exit_actions(&self) -> &[String] {
    &self.exit
  }
}

pub struct FinalStateNode {
  pub id: String,
  pub parent: String,
  pub entry: Vec<String>,
  pub exit: Vec<String>,
}
impl StateNodeTrait for FinalStateNode {
  fn id(&self) -> &str {
    &self.id
  }
  fn initial(&self) -> Option<&Transition> {
    None
  }
  fn parent(&self) -> Option<&String> {
    Some(&self.parent)
  }
  fn child_state_ids(&self) -> &[String] {
    &[]
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &[]
  }
  fn on(&self, _: &str) -> &[Transition] {
    &[]
  }
  fn history_state_ids(&self) -> &[String] {
    &[]
  }
  fn entry_actions(&self) -> &[String] {
    &self.entry
  }
  fn exit_actions(&self) -> &[String] {
    &self.exit
  }
}

pub struct ParallelStateNode {
  pub id: String,
  pub parent: String,
  pub always: Vec<Transition>,
  pub on: HashMap<String, Vec<Transition>>,
  pub initial: Transition,
  pub states: Vec<String>,
  pub history_states: Vec<String>,
  pub entry: Vec<String>,
  pub exit: Vec<String>,
}
impl StateNodeTrait for ParallelStateNode {
  fn id(&self) -> &str {
    &self.id
  }
  fn parent(&self) -> Option<&String> {
    Some(&self.parent)
  }
  fn initial(&self) -> Option<&Transition> {
    Some(&self.initial)
  }
  fn child_state_ids(&self) -> &[String] {
    &self.states
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &self.always
  }
  fn on(&self, event_name: &str) -> &[Transition] {
    match self.on.get(event_name) {
      Some(transitions) => transitions,
      None => &[],
    }
  }
  fn history_state_ids(&self) -> &[String] {
    &self.history_states
  }
  fn entry_actions(&self) -> &[String] {
    &self.entry
  }
  fn exit_actions(&self) -> &[String] {
    &self.exit
  }
}

pub enum HistoryKind {
  Shallow,
  Deep,
}
pub struct HistoryStateNode {
  pub id: String,
  pub parent: String,
  pub kind: HistoryKind,
  pub transition: Transition,
}
impl HistoryStateNode {
  pub fn target(&self) -> &Transition {
    &self.transition
  }
}
impl StateNodeTrait for HistoryStateNode {
  fn id(&self) -> &str {
    &self.id
  }
  fn initial(&self) -> Option<&'static Transition> {
    None
  }
  fn parent(&self) -> Option<&String> {
    Some(&self.parent)
  }
  fn child_state_ids(&self) -> &[String] {
    &[]
  }
  fn eventless_transitions(&self) -> &[Transition] {
    &[]
  }
  fn on(&self, _: &str) -> &[Transition] {
    &[]
  }
  fn history_state_ids(&self) -> &[String] {
    &[]
  }
  fn entry_actions(&self) -> &[String] {
    &[]
  }
  fn exit_actions(&self) -> &[String] {
    &[]
  }
}
