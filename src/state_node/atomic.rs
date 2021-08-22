use std::collections::HashSet;

use phf::OrderedMap;

use crate::{algorithm::utils, transition::Transition};

use super::{State, StateNode};

pub struct AtomicStateNode {
  pub id: &'static str,
  pub key: &'static str,
  pub parent: Option<&'static str>,
  pub on: OrderedMap<&'static str, &'static str>,
}
impl StateNode for AtomicStateNode {
  fn id(&self) -> &'static str {
    self.id
  }
  fn key(&self) -> &'static str {
    self.key
  }
  fn parent(&self) -> Option<&'static str> {
    self.parent
  }
  fn initial(&self) -> Option<Transition> {
    None
  }
  fn child_states(&self) -> Vec<&'static str> {
    vec![]
  }

  // Checks
  fn is_in_final_state(&self, _: &OrderedMap<&'static str, State>, _: &Vec<&'static str>) -> bool {
    false
  }

  // Algorithm stuff
  fn enter_state(&self, _: &mut Vec<crate::event::Event>) {
    ()
  }
  fn add_descendent_states_to_enter(
    &self,
    _: &OrderedMap<&'static str, State>,
    states_to_enter: &mut HashSet<&'static str>,
    _: &mut HashSet<&'static str>,
  ) {
    states_to_enter.insert(self.id());
  }
  fn add_ancestor_states_to_enter(
    &self,
    _: &OrderedMap<&'static str, State>,
    state_id: &'static str,
    maybe_ancestor_id: Option<&'static str>,
    states_to_enter: &mut HashSet<&'static str>,
    _: &mut HashSet<&'static str>,
  ) {
    for ancestor_id in utils::get_proper_ancestor_ids(state_id, maybe_ancestor_id) {
      states_to_enter.insert(ancestor_id);
    }
  }
  fn get_internal_transition_domain(&self, _: Vec<&str>, _: &'static str) -> Option<&'static str> {
    None
  }
  fn get_effective_target_state_ids(&self, target_ids: &mut HashSet<&'static str>) {
    target_ids.insert(self.id);
  }
}
