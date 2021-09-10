use phf::OrderedMap;
use std::collections::HashSet;

use super::{State, StateNode};
use crate::{action::Action, algorithm::utils, transition::Transition};

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

  // Algorithm stuff
  fn enter_state(&self, _: &mut Vec<crate::event::Event>) {
    ()
  }
  fn add_descendent_states_to_enter(
    &self,
    state_map: &OrderedMap<&'static str, State>,
    states_to_enter: &mut HashSet<&'static str>,
    states_for_default_entry: &mut HashSet<&'static str>,
  ) {
    states_to_enter.insert(self.id());
    states_for_default_entry.insert(self.id());

    if let Some(target_transition) = self.initial() {
      for target_state_id in target_transition.targets {
        if let Some(target_state) = state_map.get(target_state_id) {
          target_state.add_descendent_states_to_enter(
            state_map,
            states_to_enter,
            states_for_default_entry,
          );

          if let Some(target_parent_id) = target_state.parent() {
            if let Some(target_parent_state) = state_map.get(target_parent_id) {
              target_parent_state.add_ancestor_states_to_enter(
                state_map,
                target_state.id(),
                Some(target_parent_id),
                states_to_enter,
                states_for_default_entry,
              );
            }
          }
        }
      }
    }
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
  fn get_internal_transition_domain(
    &self,
    target_state_ids: Vec<&str>,
    transition_source: &'static str,
  ) -> Option<&'static str> {
    if target_state_ids
      .iter()
      .all(|s| utils::is_descendant(s, transition_source))
    {
      Some(transition_source)
    } else {
      None
    }
  }
  fn get_effective_target_state_ids(&self, target_ids: &mut HashSet<&'static str>) {
    target_ids.insert(self.id);
  }
}
