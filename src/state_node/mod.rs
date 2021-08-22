use std::{collections::HashSet, fmt};

mod atomic;
mod compound;

pub use atomic::*;
pub use compound::*;
use phf::OrderedMap;

use crate::{event::Event, transition::Transition};

pub trait StateNode {
  fn id(&self) -> &'static str;
  fn key(&self) -> &'static str;
  fn initial(&self) -> Option<Transition>;
  fn parent(&self) -> Option<&'static str>;
  fn child_states(&self) -> Vec<&'static str>;

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
impl StateNode for State {
  fn id(&self) -> &'static str {
    match self {
      Self::Atomic(s) => s.id(),
      Self::Compound(s) => s.id(),
    }
  }
  fn key(&self) -> &'static str {
    match self {
      Self::Atomic(s) => s.key(),
      Self::Compound(s) => s.key(),
    }
  }
  fn initial(&self) -> Option<Transition> {
    match self {
      Self::Atomic(s) => s.initial(),
      Self::Compound(s) => s.initial(),
    }
  }
  fn parent(&self) -> Option<&'static str> {
    match self {
      Self::Atomic(s) => s.parent(),
      Self::Compound(s) => s.parent(),
    }
  }
  fn child_states(&self) -> Vec<&'static str> {
    match self {
      Self::Atomic(s) => s.child_states(),
      Self::Compound(s) => s.child_states(),
    }
  }

  // Checks
  fn is_in_final_state(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    configuration: &Vec<&'static str>,
  ) -> bool {
    match self {
      Self::Atomic(s) => s.is_in_final_state(state_map, configuration),
      Self::Compound(s) => s.is_in_final_state(state_map, configuration),
    }
  }

  // Algorithm stuff
  fn enter_state(&self, internal_queue: &mut Vec<Event>) {
    match self {
      Self::Atomic(s) => s.enter_state(internal_queue),
      Self::Compound(s) => s.enter_state(internal_queue),
    }
  }
  fn add_descendent_states_to_enter(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    states_to_enter: &mut HashSet<&'static str>,
    states_for_default_entry: &mut HashSet<&'static str>,
  ) {
    match self {
      Self::Atomic(s) => {
        s.add_descendent_states_to_enter(state_map, states_to_enter, states_for_default_entry)
      }
      Self::Compound(s) => {
        s.add_descendent_states_to_enter(state_map, states_to_enter, states_for_default_entry)
      }
    }
  }
  fn add_ancestor_states_to_enter(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    state_id: &'static str,
    maybe_ancestor_id: Option<&'static str>,
    states_to_enter: &mut HashSet<&'static str>,
    states_for_default_entry: &mut HashSet<&'static str>,
  ) {
    match self {
      Self::Atomic(s) => s.add_ancestor_states_to_enter(
        state_map,
        state_id,
        maybe_ancestor_id,
        states_to_enter,
        states_for_default_entry,
      ),
      Self::Compound(s) => s.add_ancestor_states_to_enter(
        state_map,
        state_id,
        maybe_ancestor_id,
        states_to_enter,
        states_for_default_entry,
      ),
    }
  }
  fn get_internal_transition_domain(
    &self,
    target_state_ids: Vec<&str>,
    transition_source: &'static str,
  ) -> Option<&'static str> {
    match self {
      Self::Atomic(s) => s.get_internal_transition_domain(target_state_ids, transition_source),
      Self::Compound(s) => s.get_internal_transition_domain(target_state_ids, transition_source),
    }
  }
  fn get_effective_target_state_ids(&self, target_ids: &mut HashSet<&'static str>) {
    match self {
      Self::Atomic(s) => s.get_effective_target_state_ids(target_ids),
      Self::Compound(s) => s.get_effective_target_state_ids(target_ids),
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
