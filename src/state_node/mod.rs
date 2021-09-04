use phf::OrderedMap;
use std::{collections::HashSet, fmt, ops::Deref};

use crate::{event::Event, transition::Transition};

mod atomic;
mod compound;

pub use atomic::*;
pub use compound::*;

pub trait StateNode {
  fn id(&self) -> &'static str;
  fn key(&self) -> &'static str;
  fn initial(&self) -> Option<Transition>;
  fn parent(&self) -> Option<&'static str>;
  fn child_states(&self) -> Vec<&'static str>;
  fn transitions(&self) -> Vec<Transition>;

  // Checks
  fn is_in_final_state(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    configuration: &Vec<&'static str>,
  ) -> bool;

  // Algorithm stuff
  fn enter_state(&self, internal_queue: &mut Vec<Event>);
  fn add_descendent_states_to_enter(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    states_to_enter: &mut HashSet<&'static str>,
    states_for_default_entry: &mut HashSet<&'static str>,
  );
  fn add_ancestor_states_to_enter(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    state_id: &'static str,
    maybe_ancestor_id: Option<&'static str>,
    states_to_enter: &mut HashSet<&'static str>,
    states_for_default_entry: &mut HashSet<&'static str>,
  );
  fn get_internal_transition_domain(
    &self,
    target_state_ids: Vec<&str>,
    transition_source: &'static str,
  ) -> Option<&'static str>;
  fn get_effective_target_state_ids(&self, target_ids: &mut HashSet<&'static str>);
}

pub enum State {
  Atomic(AtomicStateNode),
  Compound(CompoundStateNode),
}
impl Deref for State {
  type Target = dyn StateNode;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Atomic(s) => s,
      Self::Compound(s) => s,
    }
  }
}
impl fmt::Debug for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Atomic(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Compound(s) => write!(f, "<StateNode \"{}\">", &s.id),
    }
  }
}
