use std::{
  collections::{HashMap, HashSet},
  iter::FromIterator,
};

use crate::{
  action::Action,
  event::Event,
  state_node::StateNode,
  transition::{Kind as TransitionKind, Transition},
};

use self::utils::{
  get_child_states, is_atomic_state, is_compound_state, is_descendant, is_final_state,
  is_history_state, is_in_final_state, is_parallel_state,
};

mod utils;

pub fn enter_states(
  state_map: &mut HashMap<String, StateNode>,
  enabled_transitions: Vec<Option<Transition>>,
  mut configuration: Vec<String>,
  mut states_to_invoke: Vec<String>,
  history_value: &HashMap<String, HashSet<String>>,
  mut actions: Vec<Action>,
  mut internal_queue: Vec<Event>,
) -> (Vec<String>, Vec<Action>, Vec<Event>) {
  let mut states_to_enter: HashSet<String> = HashSet::new();
  let mut states_for_default_entry: HashSet<String> = HashSet::new();
  let mut default_history_content: HashMap<String, String> = HashMap::new();

  compute_entry_set(
    state_map,
    enabled_transitions,
    &mut states_to_enter,
    &mut states_for_default_entry,
    &mut default_history_content,
    history_value,
  );

  // TODO: Sort... Somehow. Based on something. Ask reference
  for s in states_to_enter.into_iter() {
    &mut configuration.push(s.clone());
    states_to_invoke.push(s.clone());

    if let Some(node) = state_map.get_mut(&s) {
      let mut entry_actions = vec![];
      entry_actions.append(&mut node.entry);

      for action in entry_actions {
        execute_content(action, &mut actions, &mut internal_queue);
      }
    }
    if states_for_default_entry.contains(&s) {
      // execute_content(s.initial.transtion);
      continue;
    }
    if let Some(_value) = default_history_content.get(&s) {
      // execute_content(_value);
      continue;
    }
    if let Some(node) = state_map.get(&s) {
      if is_final_state(state_map, &s) {
        if let Some(last_dot_index_parent) = s.rfind(".") {
          let parent_id = &s[..last_dot_index_parent];

          internal_queue.push(Event {
            name: String::from("done.state.") + parent_id,
            data: match node.done_data.clone() {
              Some(d) => d,
              None => HashMap::new(),
            },
          });

          if let Some(last_dot_index_grandparent) = s.rfind(".") {
            let grandparent_id = String::from(&parent_id[..last_dot_index_grandparent]);

            if is_parallel_state(state_map, &grandparent_id) {
              if get_child_states(state_map, &grandparent_id)
                .iter()
                .all(|parent_state_id| {
                  is_in_final_state(state_map, parent_state_id, &configuration)
                })
              {
                internal_queue.push(Event {
                  name: String::from("done.state.") + &grandparent_id,
                  data: HashMap::new(),
                })
              }
            }
          }
        }
      }
    }
  }

  (configuration, actions, internal_queue)
}

fn compute_entry_set(
  state_map: &HashMap<String, StateNode>,
  transitions: Vec<Option<Transition>>,
  states_to_enter: &mut HashSet<String>,
  states_for_default_entry: &mut HashSet<String>,
  default_history_content: &mut HashMap<String, String>,
  history_value: &HashMap<String, HashSet<String>>,
) {
  for t_maybe in transitions {
    if let Some(t) = t_maybe {
      for s in t.target() {
        add_descendent_states_to_enter(
          state_map,
          s,
          states_to_enter,
          states_for_default_entry,
          default_history_content,
          history_value,
        );
      }
      let ancestor_id = get_transition_domain(state_map, &t, &history_value);
      for s in get_effective_target_state_ids(state_map, &t, &history_value) {
        add_ancestor_states_to_enter(
          state_map,
          s,
          ancestor_id.as_ref(),
          states_to_enter,
          states_for_default_entry,
          default_history_content,
          history_value,
        );
      }
    }
  }
}

fn add_descendent_states_to_enter(
  state_map: &HashMap<String, StateNode>,
  state_id: String,
  states_to_enter: &mut HashSet<String>,
  states_for_default_entry: &mut HashSet<String>,
  default_history_content: &mut HashMap<String, String>,
  history_value: &HashMap<String, HashSet<String>>,
) {
  if is_history_state(state_map, &state_id) {
    // FIXME:
    panic!("History states not supported yet!");
    // if let Some(history_value_for_state_id) = history_value.get(&state_id) {
    //   for s in history_value_for_state_id {
    //     add_descendent_states_to_enter(
    //       state_map,
    //       s.clone(),
    //       states_to_enter,
    //       states_for_default_entry,
    //       default_history_content,
    //       history_value,
    //     );
    //     if let Some(last_dot_index) = s.rfind(".") {
    //       add_ancestor_states_to_enter(
    //         state_map,
    //         s.clone(),
    //         &String::from(&s[..last_dot_index]),
    //         states_to_enter,
    //         states_for_default_entry,
    //         default_history_content,
    //         history_value,
    //       )
    //     }
    //   }
    // } else {
    //   // Not really sure what goes here, it's commented out in the reference as well
    // }
  } else {
    states_to_enter.insert(state_id.clone());
    if is_compound_state(state_map, &state_id) {
      states_for_default_entry.insert(state_id.clone());
      if let Some(node) = state_map.get(&state_id) {
        if let Some(target) = node.initial() {
          for s in target.target() {
            add_descendent_states_to_enter(
              state_map,
              s.clone(),
              states_to_enter,
              states_for_default_entry,
              default_history_content,
              history_value,
            );
            if let Some(last_dot_index) = s.rfind(".") {
              add_ancestor_states_to_enter(
                state_map,
                s.clone(),
                Some(&String::from(&s[..last_dot_index])),
                states_to_enter,
                states_for_default_entry,
                default_history_content,
                history_value,
              );
            }
          }
        }
      }
    } else {
      if is_parallel_state(state_map, &state_id) {
        for child in get_child_states(state_map, &state_id) {
          if states_to_enter.iter().any(|s| is_descendant(s, &child)) {
            add_descendent_states_to_enter(
              state_map,
              child,
              states_to_enter,
              states_for_default_entry,
              default_history_content,
              history_value,
            )
          }
        }
      }
    }
  }
}

fn get_transition_domain(
  state_map: &HashMap<String, StateNode>,
  transition: &Transition,
  history_value: &HashMap<String, HashSet<String>>,
) -> Option<String> {
  let mut tstates = get_effective_target_state_ids(state_map, transition, history_value);

  if tstates.is_empty() {
    None
  } else if let TransitionKind::Internal = transition.kind {
    if is_compound_state(state_map, &transition.source)
      && tstates.iter().all(|s| is_descendant(s, &transition.source))
    {
      Some(transition.source.clone())
    } else {
      None
    }
  } else {
    let mut lcca_state_ids = vec![transition.source.clone()];
    lcca_state_ids.append(&mut tstates);

    find_lcca(lcca_state_ids)
  }
}

fn find_lcca(state_id_list: Vec<String>) -> Option<String> {
  let mut lcca = None;

  for anc in get_proper_ancestors(state_id_list[0].clone(), None) {
    if state_id_list[1..].iter().all(|s| is_descendant(s, &anc)) {
      lcca = Some(anc);
    }
  }

  lcca
}

fn get_effective_target_state_ids(
  state_map: &HashMap<String, StateNode>,
  transition: &Transition,
  _history_value: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
  let mut targets: HashSet<String> = HashSet::new();

  for s in transition.target() {
    if is_history_state(state_map, &s) {
      // FIXME:
      panic!("History states not supported yet!");
      // if let Some(val) = history_value.get(&s) {
      //   targets.insert(val.clone());
      // } else {
      //   if let Some(s_node) = state_map.get(s) {
      //     targets.extend(get_effective_target_states(
      //       state_map,
      //       s_node.transition,
      //       history_value,
      //     ));
      //   }
      // }
    } else {
      targets.insert(s);
    }
  }

  Vec::from_iter(targets)
}

fn add_ancestor_states_to_enter(
  state_map: &HashMap<String, StateNode>,
  state_id: String,
  ancestor_id: Option<&String>,
  states_to_enter: &mut HashSet<String>,
  states_for_default_entry: &mut HashSet<String>,
  default_history_content: &mut HashMap<String, String>,
  history_value: &HashMap<String, HashSet<String>>,
) {
  for anc in get_proper_ancestors(state_id, ancestor_id) {
    states_to_enter.insert(anc.clone());
    if is_parallel_state(state_map, &anc) {
      for child in get_child_states(state_map, &anc) {
        if !(states_to_enter.iter().any(|s| is_descendant(s, &child))) {
          add_descendent_states_to_enter(
            state_map,
            child,
            states_to_enter,
            states_for_default_entry,
            default_history_content,
            history_value,
          );
        }
      }
    }
  }
}

fn get_proper_ancestors(state_id: String, ancestor_id_maybe: Option<&String>) -> Vec<String> {
  let mut ancestors = vec![];

  if let Some(last_dot_index) = state_id.rfind(".") {
    let intermediates_string = &state_id[..last_dot_index];
    let intermediates = intermediates_string.split(".");

    if let Some(ancestor_id) = ancestor_id_maybe {
      let mut step = ancestor_id.clone();

      for i in intermediates {
        if i != ancestor_id {
          let ancestor = step + "." + i;
          ancestors.push(ancestor.clone());
          step = ancestor;
        }
      }
    }
  }

  ancestors
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

//   let mut isLooping = true;
//   for state_id in atomic_states {
//     if isLooping == false {
//       break;
//     }

//     let mut proper_ancestors = get_proper_ancestors(state_id.clone(), None);
//     let mut ancestors = vec![state_id.clone()];
//     ancestors.append(&mut proper_ancestors);

//     for s in ancestors {
//       if let Some(node) = state_map.get(&s) {
//         for t in &node.transitions {
//           if !t.event.is_empty() && condition_match(t) {
//             enabled_transitions.push(t.clone());
//           }
//         }
//       }
//     }
//   }

//   enabled_transitions
// }
