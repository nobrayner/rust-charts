use phf::OrderedMap;
use std::collections::{HashMap, VecDeque};

use crate::{
  action::Action,
  event::Event,
  state::State,
  state_node::State as StateNode,
  transition::{Transition, TransitionKind},
};

pub mod utils;

pub fn initial_state(
  state_map: &OrderedMap<&'static str, StateNode>,
  initial_transition: &'static Transition,
) -> State {
  let mut internal_queue = VecDeque::new();
  let mut configuration = vec![];
  // let mut states_to_invoke = vec![];

  let transitions = vec![initial_transition];

  let actions = enter_states(
    state_map,
    &transitions,
    &mut configuration,
    &mut internal_queue,
  );

  // Step 1: Macro step
  macrostep(state_map, &mut configuration, &mut internal_queue);

  // Step 2: Invoke
  // TODO: Invoke states_to_invoke

  // Step 3: Repeat step 1 for any invoke errors
  // TODO: perform another event loop step if there are errors?
  // if !internal_queue.is_empty() { macrostep(); }

  State {
    configuration,
    // configuration,
    actions,
  }
}

/*
The ordering of these steps seems "incorrect"...

According to SCXML spec, internal_queue is a global, and so it can be added to,
then processed on the next pass of the main event loop. However, if the step
order is changed, the internal_queue looks like it can be made to only relate to
the given cycle of the event loop. This would also change how starting the
interpreter works... Will need to enter states, then perform:
macrostep, invoke, macrostop for invoke errors

"Better order" would be:
process external event, macrostep, invoke, macrostep for invoke errors
*/
pub fn event_loop_step(
  state_map: &OrderedMap<&'static str, StateNode>,
  current_state: State,
  triggered_event: Event,
) -> State {
  let mut internal_queue = VecDeque::new();
  let mut actions = vec![];

  let mut configuration = current_state.configuration;

  // Step 4: Process external event
  // TODO: Cancel event performs any cleanup that needs to occur
  // if is_cancel_event(event) { event_loop_step(state, event); }

  // Invoke anything based on the event

  let enabled_transitions = select_transitions(state_map, triggered_event, &configuration);

  if !enabled_transitions.is_empty() {
    let mut microstep_actions = microstep(
      state_map,
      &enabled_transitions,
      &mut configuration,
      &mut internal_queue,
    );

    actions.append(&mut microstep_actions);
  }

  // Step 1: Macro step
  let mut macrostep_actions = macrostep(state_map, &mut configuration, &mut internal_queue);
  actions.append(&mut &mut macrostep_actions);

  // Step 2: Invoke
  // TODO: Invoke states_to_invoke

  // Step 3: Repeat step 1 for any invoke errors
  // TODO: perform another event loop step if there are errors?
  // if !internal_queue.is_empty() { macrostep(); }

  State {
    configuration,
    actions,
    ..current_state
  }
}

fn macrostep(
  state_map: &OrderedMap<&'static str, StateNode>,
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) -> Vec<&'static Action> {
  let mut enabled_transitions;
  let mut actions = vec![];

  let mut done = false;

  while done == false {
    enabled_transitions = select_eventless_transitions(state_map, configuration);

    if enabled_transitions.is_empty() {
      if internal_queue.is_empty() {
        done = true;
      } else {
        let maybe_event = internal_queue.pop_front();
        if let Some(internal_event) = maybe_event {
          enabled_transitions = select_transitions(state_map, internal_event, &configuration);
        }
      }
    }
    if !enabled_transitions.is_empty() {
      let mut microstep_actions = microstep(
        state_map,
        &enabled_transitions,
        configuration,
        internal_queue,
      );

      actions.append(&mut microstep_actions);
    }
  }

  actions
}

fn microstep(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) -> Vec<&'static Action> {
  let exit_actions = exit_states(state_map, enabled_transitions, configuration);

  // TODO: Accumulate transition actions
  let transition_actions = enabled_transitions.iter().fold(vec![], |mut actions, &t| {
    for &action in t.actions {
      actions.push(action);
    }

    actions
  });

  let enter_actions = enter_states(
    state_map,
    enabled_transitions,
    configuration,
    internal_queue,
  );

  vec![exit_actions, transition_actions, enter_actions]
    .into_iter()
    .flatten()
    .collect()
}

fn select_eventless_transitions(
  state_map: &OrderedMap<&'static str, StateNode>,
  configuration: &[&'static str],
) -> Vec<&'static Transition> {
  let mut enabled_transitions = vec![];
  // TODO: document order sort?
  let atomic_states: Vec<_> = configuration
    .iter()
    .filter(|&&state_id| {
      if let Some(state) = state_map.get(state_id) {
        match state {
          StateNode::Atomic(_) => true,
          _ => false,
        }
      } else {
        false
      }
    })
    .collect();

  for &atomic_state_id in atomic_states {
    let mut state_and_ancestor_ids = vec![atomic_state_id];
    state_and_ancestor_ids.append(&mut utils::get_proper_ancestor_ids(
      state_map,
      atomic_state_id,
      None,
    ));

    let mut looping = true;
    for state_id in state_and_ancestor_ids {
      if looping == false {
        break;
      }

      if let Some(state) = state_map.get(state_id) {
        for transition in state.eventless_transitions() {
          if utils::guard_match(transition) {
            enabled_transitions.push(transition);
            looping = false;
          }
        }
      }
    }
  }

  remove_conflicting_transitions(state_map, enabled_transitions, configuration)
}

fn select_transitions(
  state_map: &OrderedMap<&'static str, StateNode>,
  event: Event,
  configuration: &[&'static str],
) -> Vec<&'static Transition> {
  let mut enabled_transitions = vec![];
  // TODO: document order sort?
  let atomic_states: Vec<_> = configuration
    .iter()
    .filter(|&&state_id| {
      if let Some(state) = state_map.get(state_id) {
        match state {
          StateNode::Atomic(_) => true,
          _ => false,
        }
      } else {
        false
      }
    })
    .collect();

  for &atomic_state_id in atomic_states {
    let mut state_and_ancestor_ids = vec![atomic_state_id];
    state_and_ancestor_ids.append(&mut utils::get_proper_ancestor_ids(
      state_map,
      atomic_state_id,
      None,
    ));

    let mut looping = true;
    for state_id in state_and_ancestor_ids {
      if looping == false {
        break;
      }

      if let Some(state) = state_map.get(state_id) {
        for transition in state.on(&event.name) {
          if utils::guard_match(transition) {
            enabled_transitions.push(transition);
            looping = false;
          }
        }
      }
    }
  }

  remove_conflicting_transitions(state_map, enabled_transitions, configuration)
}

fn remove_conflicting_transitions(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: Vec<&'static Transition>,
  configuration: &[&'static str],
) -> Vec<&'static Transition> {
  let mut filtered_transitions = vec![];

  for t1 in enabled_transitions {
    let mut t1_preempted = false;
    let mut transitions_to_remove = vec![];

    for &t2 in &filtered_transitions {
      let t1_exit_set = compute_exit_set(state_map, &vec![t1], configuration);
      let t2_exit_set = compute_exit_set(state_map, &vec![t2], configuration);

      let has_intersection = t1_exit_set.iter().any(|t| t2_exit_set.contains(t))
        || t2_exit_set.iter().any(|t| t1_exit_set.contains(t));

      if has_intersection {
        if utils::is_descendant(state_map, t1.source, t2.source) {
          transitions_to_remove.push(t2);
        } else {
          t1_preempted = true;
          break;
        }
      }
    }

    if t1_preempted == false {
      for t3 in transitions_to_remove {
        let maybe_index = filtered_transitions.iter().position(|&t| t == t3);
        if let Some(index) = maybe_index {
          filtered_transitions.remove(index);
        }
      }

      filtered_transitions.push(t1);
    }
  }

  filtered_transitions
}

fn exit_states(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &mut Vec<&'static str>,
) -> Vec<&'static Action> {
  let state_ids_to_exit = compute_exit_set(state_map, enabled_transitions, configuration);
  let mut actions = vec![];

  for &_state_id in &state_ids_to_exit {
    // TODO: states_to_invoke
    // states_to_invoke.remove(states_to_invoke.iter().position(|&s| s == _state_id));
  }

  // TODO: Sort by `exit_order`
  // state_ids_to_exit = state_ids_to_exit.sort_by(exit_order);

  for &state_id in &state_ids_to_exit {
    if let Some(_state) = state_map.get(state_id) {
      // TODO: History States
      // for (history_id, history_kind) in state.histories() {
      //   let filter = match history_kind {
      //     HistoryKind::Deep => |s| match s {
      //       StateNode::Atomic(_) => utils::is_descendant(state_map, s.id(), state_id),
      //       _ => false,
      //     }
      //   }
      // }
    }
  }
  for &state_id in &state_ids_to_exit {
    if let Some(state) = state_map.get(state_id) {
      for action in state.exit_actions() {
        actions.push(action);
      }

      // TODO: Invoking stuff
      // for inv in state.invoke() {
      //   cancel_invoke(inv);
      // }

      let maybe_state_id_index = configuration.iter().position(|&s| s == state_id);
      if let Some(state_id_index) = maybe_state_id_index {
        configuration.remove(state_id_index);
      }
    }
  }

  actions
}

fn compute_exit_set(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &[&'static str],
) -> Vec<&'static str> {
  let mut state_ids_to_exit = vec![];

  for &transition in enabled_transitions {
    if !transition.targets.is_empty() {
      let maybe_domain = get_transition_domain(state_map, transition);

      if let Some(domain) = maybe_domain {
        for &state_id in configuration {
          if utils::is_descendant(state_map, state_id, domain) {
            state_ids_to_exit.push(state_id);
          }
        }
      }
    }
  }

  state_ids_to_exit
}

fn enter_states(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) -> Vec<&'static Action> {
  let (state_ids_to_enter, state_ids_for_default_entry, default_history_actions) =
    compute_entry_set(state_map, enabled_transitions);
  let mut actions_to_execute = vec![];

  // TODO: Sort by `entry_order`
  for state_id in state_ids_to_enter {
    if let Some(state) = state_map.get(state_id) {
      configuration.push(state_id);
      // TODO: states_to_invoke
      // states_to_invoke.push(state_id);

      for action in state.entry_actions() {
        actions_to_execute.push(action);
      }
      if state_ids_for_default_entry.contains(&state_id) {
        if let Some(transition) = state.initial() {
          for action in transition.actions {
            actions_to_execute.push(action);
          }
        }
      }
      if let Some(&actions) = default_history_actions.get(state_id) {
        for &action in actions {
          actions_to_execute.push(action);
        }
      }

      match state {
        StateNode::Final(_) => {
          if let Some(parent_id) = state.parent() {
            internal_queue.push_back(Event {
              name: String::from("done.state.") + parent_id,
              data: HashMap::new(),
            });

            if let Some(parent) = state_map.get(parent_id) {
              if let Some(grandparent_id) = parent.parent() {
                if let Some(StateNode::Parallel(_grandparent)) = state_map.get(grandparent_id) {
                  // TODO: Parallel states
                  // if grandparent
                  //   .child_state_ids()
                  //   .into_iter()
                  //   .all(|child_state_id| {
                  //     utils::is_in_final_state(state_map, &configuration, child_state_id)
                  //   })
                  // {
                  //   internal_queue.push_back(Event {
                  //     name: String::from("done.state.") + grandparent_id,
                  //     data: HashMap::new(),
                  //   })
                  // }
                }
              }
            }
          }
        }
        _ => (),
      };
    }
  }

  actions_to_execute
}

fn compute_entry_set(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
) -> (
  Vec<&'static str>,
  Vec<&'static str>,
  HashMap<&'static str, &'static [&'static Action]>,
) {
  let mut state_ids_to_enter = vec![];
  let mut state_ids_for_default_entry = vec![];
  let mut default_history_actions = HashMap::new();

  for &transition in enabled_transitions {
    for &state_id in transition.targets {
      add_descendent_states_to_enter(
        state_map,
        state_id,
        &mut state_ids_to_enter,
        &mut state_ids_for_default_entry,
        &mut default_history_actions,
      );
    }

    let maybe_ancestor_id = get_transition_domain(state_map, transition);

    if let Some(ancestor_id) = maybe_ancestor_id {
      for state_id in get_effective_target_states(state_map, transition) {
        add_ancestor_states_to_enter(
          state_map,
          state_id,
          ancestor_id,
          &mut state_ids_to_enter,
          &mut state_ids_for_default_entry,
          &mut default_history_actions,
        );
      }
    }
  }

  (
    state_ids_to_enter,
    state_ids_for_default_entry,
    default_history_actions,
  )
}

fn add_descendent_states_to_enter(
  state_map: &OrderedMap<&'static str, StateNode>,
  state_id: &'static str,
  state_ids_to_enter: &mut Vec<&'static str>,
  state_ids_for_default_entry: &mut Vec<&'static str>,
  default_history_actions: &mut HashMap<&'static str, &'static [&'static Action]>,
) {
  // FIXME:
  let history_value: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

  if let Some(state) = state_map.get(state_id) {
    match state {
      StateNode::History(_) => {
        if let Some(history_state_ids) = history_value.get(state_id) {
          for history_state_id in history_state_ids {
            add_descendent_states_to_enter(
              state_map,
              history_state_id,
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
            );
          }
          if let Some(parent_id) = state.parent() {
            for history_state_id in history_state_ids {
              add_ancestor_states_to_enter(
                state_map,
                history_state_id,
                parent_id,
                state_ids_to_enter,
                state_ids_for_default_entry,
                default_history_actions,
              )
            }
          }
        } else {
          // History states have one transition that is required
          let transition = state.transitions()[0];

          for &target_state_id in transition.targets {
            add_descendent_states_to_enter(
              state_map,
              target_state_id,
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
            );
          }

          if let Some(parent_id) = state.parent() {
            default_history_actions.insert(parent_id, transition.actions);

            for &target_state_id in transition.targets {
              add_ancestor_states_to_enter(
                state_map,
                target_state_id,
                parent_id,
                state_ids_to_enter,
                state_ids_for_default_entry,
                default_history_actions,
              );
            }
          }
        }
      }
      StateNode::Compound(_) => {
        state_ids_to_enter.push(state_id);
        state_ids_for_default_entry.push(state_id);

        if let Some(transition) = state.initial() {
          for &target_state_id in transition.targets {
            add_descendent_states_to_enter(
              state_map,
              target_state_id,
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
            );
          }

          for &target_state_id in transition.targets {
            add_ancestor_states_to_enter(
              state_map,
              target_state_id,
              state_id,
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
            );
          }
        }
      }
      StateNode::Parallel(_) => {
        state_ids_to_enter.push(state_id);

        for &child_id in state.child_state_ids() {
          if !state_ids_to_enter
            .iter()
            .any(|&s| utils::is_descendant(state_map, s, child_id))
          {
            add_descendent_states_to_enter(
              state_map,
              child_id,
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
            );
          }
        }
      }
      _ => state_ids_to_enter.push(state_id),
    }
  }
}

fn add_ancestor_states_to_enter(
  state_map: &OrderedMap<&'static str, StateNode>,
  state_id: &'static str,
  ancestor_id: &'static str,
  state_ids_to_enter: &mut Vec<&'static str>,
  state_ids_for_default_entry: &mut Vec<&'static str>,
  default_history_actions: &mut HashMap<&'static str, &'static [&'static Action]>,
) {
  for ancestor_id in utils::get_proper_ancestor_ids(state_map, state_id, Some(ancestor_id)) {
    if let Some(ancestor) = state_map.get(ancestor_id) {
      state_ids_to_enter.push(ancestor_id);

      match ancestor {
        StateNode::Parallel(_) => {
          for &child_id in ancestor.child_state_ids() {
            if !state_ids_to_enter
              .iter()
              .any(|&s| utils::is_descendant(state_map, s, child_id))
            {
              add_descendent_states_to_enter(
                state_map,
                child_id,
                state_ids_to_enter,
                state_ids_for_default_entry,
                default_history_actions,
              );
            }
          }
        }
        _ => (),
      }
    }
  }
}

fn get_transition_domain(
  state_map: &OrderedMap<&'static str, StateNode>,
  transition: &Transition,
) -> Option<&'static str> {
  let transition_state_ids = get_effective_target_states(state_map, transition);

  let lcca = |mut transition_state_ids| {
    let mut state_list = vec![transition.source];
    state_list.append(&mut transition_state_ids);

    find_lcca(state_map, state_list)
  };

  if transition_state_ids.is_empty() {
    None
  } else {
    match transition.kind {
      TransitionKind::Internal => {
        if let Some(source_state) = state_map.get(transition.source) {
          match source_state {
            StateNode::Compound(_) => {
              if transition_state_ids
                .iter()
                .all(|&s| utils::is_descendant(state_map, s, transition.source))
              {
                Some(transition.source)
              } else {
                lcca(transition_state_ids)
              }
            }
            _ => lcca(transition_state_ids),
          }
        } else {
          lcca(transition_state_ids)
        }
      }
      _ => lcca(transition_state_ids),
    }
  }
}

fn get_effective_target_states(
  state_map: &OrderedMap<&'static str, StateNode>,
  transition: &Transition,
) -> Vec<&'static str> {
  // FIXME:
  let history_value: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

  let mut targets = vec![];

  for &target_state_id in transition.targets {
    if let Some(target_state) = state_map.get(target_state_id) {
      match target_state {
        StateNode::History(_) => {
          if let Some(history_state_ids) = history_value.get(target_state_id) {
            history_state_ids
              .iter()
              .for_each(|&state_id| targets.push(state_id))
          } else {
            get_effective_target_states(state_map, target_state.transitions()[0])
              .iter()
              .for_each(|&state_id| targets.push(state_id));
          }
        }
        _ => targets.push(target_state_id),
      }
    }
  }

  targets
}

fn find_lcca(
  state_map: &OrderedMap<&'static str, StateNode>,
  state_list: Vec<&'static str>,
) -> Option<&'static str> {
  let mut lcca = None;

  for &ancestor_id in utils::get_proper_ancestor_ids(state_map, state_list[0], None)
    .iter()
    .filter(|&&state_id| {
      if let Some(state) = state_map.get(state_id) {
        match state {
          // The root node also counts as an LCCA
          StateNode::Root(_) => true,
          StateNode::Compound(_) => true,
          _ => false,
        }
      } else {
        false
      }
    })
  {
    if state_list[1..]
      .iter()
      .all(|&s| utils::is_descendant(state_map, s, ancestor_id))
    {
      lcca = Some(ancestor_id);
    }
  }

  lcca
}
