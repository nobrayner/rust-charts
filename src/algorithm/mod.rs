use std::{
  collections::{HashMap, HashSet},
  iter::FromIterator,
};

use phf::OrderedMap;

use crate::{
  action::Action,
  event::Event,
  state_node::{State, StateNode},
  transition::{Kind as TransitionKind, Transition},
};

pub mod utils;

pub fn enter_states<'s>(
  state_map: &OrderedMap<&'s str, State>,
  enabled_transitions: Vec<Option<Transition>>,
  mut configuration: Vec<&'s str>,
  mut states_to_invoke: Vec<&str>,
  history_value: &HashMap<String, HashSet<String>>,
  mut actions: Vec<Action>,
  mut internal_queue: Vec<Event>,
) -> (Vec<&'s str>, Vec<Action>, Vec<Event>) {
  let mut states_to_enter = HashSet::new();
  let mut states_for_default_entry = HashSet::new();
  let mut default_history_content = HashMap::new();

  compute_entry_set(
    state_map,
    enabled_transitions,
    &mut states_to_enter,
    &mut states_for_default_entry,
    &mut default_history_content,
    history_value,
  );

  // TODO: Sort... Somehow. Based on something. Ask reference
  for state_id in states_to_enter.into_iter() {
    &mut configuration.push(state_id);
    states_to_invoke.push(state_id);

    if let Some(state) = state_map.get(state_id) {
      //   for action in &state.entry_actions() {
      //     execute_content(action.clone(), &mut actions, &mut internal_queue);
      //   }

      if states_for_default_entry.contains(state_id) {
        // execute_content(s.initial.transtion);
      }
      if let Some(_value) = default_history_content.get(state_id) {
        // execute_content(_value);
      }
      state.enter_state(&mut internal_queue);
    }
  }

  (configuration, actions, internal_queue)
}

fn compute_entry_set(
  state_map: &OrderedMap<&str, State>,
  transitions: Vec<Option<Transition>>,
  states_to_enter: &mut HashSet<&'static str>,
  states_for_default_entry: &mut HashSet<&'static str>,
  default_history_content: &mut HashMap<String, String>,
  history_value: &HashMap<String, HashSet<String>>,
) {
  for maybe_transition in transitions {
    if let Some(t) = maybe_transition {
      for target_state_id in t.target() {
        if let Some(target_state) = state_map.get(&target_state_id) {
          target_state.add_descendent_states_to_enter(
            state_map,
            states_to_enter,
            states_for_default_entry,
          );
        }
      }
      let maybe_ancestor_id = get_transition_domain(state_map, &t, &history_value);
      for target_state_id in get_effective_target_state_ids(state_map, &t, &history_value) {
        if let Some(target_state) = state_map.get(&target_state_id) {
          target_state.add_ancestor_states_to_enter(
            state_map,
            target_state_id,
            maybe_ancestor_id,
            states_to_enter,
            states_for_default_entry,
            // default_history_content,
            // history_value,
          );
        }
      }
    }
  }
}

fn get_transition_domain<'s>(
  state_map: &OrderedMap<&'s str, State>,
  transition: &Transition,
  history_value: &HashMap<String, HashSet<String>>,
) -> Option<&'s str> {
  let mut target_state_ids = get_effective_target_state_ids(state_map, transition, history_value);

  if target_state_ids.is_empty() {
    None
  } else if let TransitionKind::Internal = transition.kind {
    if let Some(transition_source_state) = state_map.get(transition.source) {
      transition_source_state.get_internal_transition_domain(target_state_ids, transition.source)
    } else {
      None
    }
  } else {
    let mut lcca_state_ids = vec![transition.source];
    lcca_state_ids.append(&mut target_state_ids);

    find_lcca(lcca_state_ids)
  }
}

fn find_lcca(state_id_list: Vec<&str>) -> Option<&str> {
  let mut lcca = None;

  for ancestor_id in utils::get_proper_ancestor_ids(state_id_list[0], None) {
    if state_id_list[1..]
      .iter()
      .all(|state_id| utils::is_descendant(state_id, ancestor_id))
    {
      lcca = Some(ancestor_id);
    }
  }

  lcca
}

fn get_effective_target_state_ids<'s>(
  state_map: &OrderedMap<&'s str, State>,
  transition: &Transition,
  _history_value: &HashMap<String, HashSet<String>>,
) -> Vec<&'s str> {
  let mut target_state_ids = HashSet::new();

  for state_id in transition.target() {
    if let Some(state) = state_map.get(state_id) {
      state.get_effective_target_state_ids(&mut target_state_ids);
    }
  }

  Vec::from_iter(target_state_ids)
}

fn execute_content(action: Action, actions: &mut Vec<Action>, internal_queue: &mut Vec<Event>) {
  if action.kind == "xstate:raise" {
    internal_queue.push(Event {
      name: match action.data.get("event") {
        Some(name) => name.clone(),
        None => String::from(""),
      },
      data: HashMap::new(),
    });
  } else {
    actions.push(action);
  }
}

// LIFECYCLE

// pub fn macrostep(
//   state_map: &HashMap<String, StateNode>,
//   configuration: Vec<String>,
//   actions: Vec<Action>,
//   mut internal_queue: Vec<Event>,
// ) -> (Vec<String>, Vec<Action>) {
//   let mut enabled_transitions: Vec<Transition> = vec![];
//   let mut macrostep_done = false;

//   while !macrostep_done {
//     {
//       enabled_transitions = select_eventless_transitions(state_map, &configuration);
//     }

//     if enabled_transitions.is_empty() {
//       if internal_queue.is_empty() {
//         macrostep_done = true;
//       } else {
//         let internal_event = internal_queue.pop();
//         // enabled_transitions = select_transitions(internal_event, &mut configuration);
//       }
//     }
//     if !enabled_transitions.is_empty() {
//       // let (configuration, actions, internal_queue) = microstep(
//       //   enabled_transitions,
//       //   configuration,
//       //   HashSet::new(), // TODO:
//       //   HashMap::new(), // TODO:
//       // );
//     }
//   }

//   (configuration, actions)
// }

// fn select_eventless_transitions(
//   state_map: &HashMap<String, StateNode>,
//   configuration: &Vec<String>,
// ) -> Vec<Transition> {
//   let mut enabled_transitions: Vec<Transition> = vec![];
//   let atomic_states = configuration
//     .iter()
//     .filter(|s| is_atomic_state(state_map, s));

//   let mut is_looping = true;
//   for state_id in atomic_states {
//     if is_looping == false {
//       break;
//     }

//     let mut proper_ancestors = get_proper_ancestors(state_id.clone(), None);
//     let mut ancestors = vec![state_id.clone()];
//     ancestors.append(&mut proper_ancestors);

//     for s in ancestors {
//       if let Some(node) = state_map.get(&s) {
//         for t in &node.transitions.clone() {
//           if !t.event.is_empty() && condition_match(t) {
//             enabled_transitions.push(t.clone());
//           }
//         }
//       }
//     }
//   }

//   let enabled_transitions = remove_conflicting_transitions(
//     enabled_transitions,
//     configuration,
//     // TODO:
//     &HashMap::new(),
//   );

//   enabled_transitions
// }

// fn remove_conflicting_transitions(
//   enabled_transitions: Vec<Transition>,
//   configuration: &Vec<String>,
//   history_value: &HashMap<String, HashSet<String>>,
// ) -> Vec<Transition> {
//   let enabled_transition = enabled_transitions.sort_by(|a, b| a.order.cmp(&b.order));
//   let filtered_transitions: HashSet<Transition> = HashSet::new();

//   for t1 in enabled_transitions {
//     let t1_preempted = false;
//     let transitions_to_remove = HashSet::new();

//     for t2 in filtered_transitions {
//       let t1_exit_set = compute_exit_set(vec![&t1], configuration, history_value);
//       let t2_exit_set = compute_exit_set(vec![&t2], configuration, history_value);

//       let intersection = t1_exit_set
//     }
//   }

//   Vec::from(filtered_transitions)
// }
