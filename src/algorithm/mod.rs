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
    value: configuration,
    // configuration,
    actions,
  }
}

/*
FIXME: The ordering of these steps seems "incorrect"...

According to SCXML spec, internal_queue is a global, and so it can be added to,
then processed on the next pass of the main event loop. However, if the step
order is changed, the internal_queue looks like it can be made to only relate to
the given cycle of the event loop. This would also change how starting the
interpreter works... Will need to enter states, then perform:
macrostep, invoke, macrostop for invoke errors

"Better order" would be:
process external event, macrostep, invoke, macrostep for invoke errors
*/
fn event_loop_step(
  state_map: &OrderedMap<&'static str, StateNode>,
  current_state: State,
  triggered_event: Event,
) -> State {
  let mut internal_queue = VecDeque::new();
  let mut configuration = current_state.value;

  // Step 1: Macro step
  macrostep(state_map, &mut &mut configuration, &mut internal_queue);

  // Step 2: Invoke
  // TODO: Invoke states_to_invoke

  // Step 3: Repeat step 1 for any invoke errors
  // TODO: perform another event loop step if there are errors?
  // if !internal_queue.is_empty() { macrostep(); }

  // Step 4: Process external event
  // TODO: Cancel event performs any cleanup that needs to occur
  // if is_cancel_event(event) { event_loop_step(state, event); }

  // Invoke anything based on the event

  let enabled_transitions = select_transitions(triggered_event);

  if !enabled_transitions.is_empty() {
    microstep(
      state_map,
      &enabled_transitions,
      &mut configuration,
      &mut internal_queue,
    );
  }

  // FIXME: Set proper state values
  State {
    value: configuration,
    ..current_state
  }
}

fn macrostep(
  state_map: &OrderedMap<&'static str, StateNode>,
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) {
  let mut enabled_transitions;

  let mut done = false;

  while done == false {
    enabled_transitions = select_eventless_transitions();

    if enabled_transitions.is_empty() {
      if internal_queue.is_empty() {
        done = true;
      } else {
        let maybe_event = internal_queue.pop_front();
        if let Some(internal_event) = maybe_event {
          enabled_transitions = select_transitions(internal_event);
        }
      }
    }
    if !enabled_transitions.is_empty() {
      microstep(
        state_map,
        &enabled_transitions,
        configuration,
        internal_queue,
      );
    }
  }
}

fn microstep(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) {
  exit_states(enabled_transitions);
  // execute_transition_content(enabled_transitions);
  enter_states(
    state_map,
    enabled_transitions,
    configuration,
    internal_queue,
  );
}

fn select_eventless_transitions() -> Vec<&'static Transition> {
  // TODO: get each state node's `always()` transitions
  vec![]
}

fn select_transitions(_event: Event) -> Vec<&'static Transition> {
  // TODO:
  vec![]
}

fn exit_states(_enabled_transitions: &[&Transition]) {
  // TODO:
}

fn enter_states(
  state_map: &OrderedMap<&'static str, StateNode>,
  enabled_transitions: &[&Transition],
  configuration: &mut Vec<&'static str>,
  internal_queue: &mut VecDeque<Event>,
) -> Vec<&'static Action> {
  let (states_to_enter, states_for_default_entry, default_history_actions) =
    compute_entry_set(state_map, enabled_transitions);
  let mut actions_to_execute = vec![];

  for state_id in states_to_enter {
    if let Some(state) = state_map.get(state_id) {
      configuration.push(state_id);
      // states_to_invoke.push(state_id);

      for action in state.entry_actions() {
        actions_to_execute.push(action);
      }
      if states_for_default_entry.contains(&state_id) {
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
  let mut states_to_enter = vec![];
  let mut states_for_default_entry = vec![];
  let mut default_history_actions = HashMap::new();

  for &transition in enabled_transitions {
    for &state_id in transition.targets {
      add_descendent_states_to_enter(
        state_map,
        state_id,
        &mut states_to_enter,
        &mut states_for_default_entry,
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
          &mut states_to_enter,
          &mut states_for_default_entry,
          &mut default_history_actions,
        );
      }
    }
  }

  (
    states_to_enter,
    states_for_default_entry,
    default_history_actions,
  )
}

fn add_descendent_states_to_enter(
  state_map: &OrderedMap<&'static str, StateNode>,
  state_id: &'static str,
  states_to_enter: &mut Vec<&'static str>,
  states_for_default_entry: &mut Vec<&'static str>,
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
              states_to_enter,
              states_for_default_entry,
              default_history_actions,
            );
          }
          if let Some(parent_id) = state.parent() {
            for history_state_id in history_state_ids {
              add_ancestor_states_to_enter(
                state_map,
                history_state_id,
                parent_id,
                states_to_enter,
                states_for_default_entry,
                default_history_actions,
              )
            }
          }
        } else {
          let transition = state.transitions()[0];

          for &target_state_id in transition.targets {
            add_descendent_states_to_enter(
              state_map,
              target_state_id,
              states_to_enter,
              states_for_default_entry,
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
                states_to_enter,
                states_for_default_entry,
                default_history_actions,
              );
            }
          }
        }
      }
      StateNode::Compound(_) => {
        states_to_enter.push(state_id);
        states_for_default_entry.push(state_id);

        if let Some(transition) = state.initial() {
          for &target_state_id in transition.targets {
            add_descendent_states_to_enter(
              state_map,
              target_state_id,
              states_to_enter,
              states_for_default_entry,
              default_history_actions,
            );
          }

          for &target_state_id in transition.targets {
            add_ancestor_states_to_enter(
              state_map,
              target_state_id,
              state_id,
              states_to_enter,
              states_for_default_entry,
              default_history_actions,
            );
          }
        }
      }
      StateNode::Parallel(_) => {
        states_to_enter.push(state_id);

        for &child_id in state.child_state_ids() {
          if !states_to_enter
            .iter()
            .any(|&s| utils::is_descendant(s, child_id))
          {
            add_descendent_states_to_enter(
              state_map,
              child_id,
              states_to_enter,
              states_for_default_entry,
              default_history_actions,
            );
          }
        }
      }
      _ => states_to_enter.push(state_id),
    }
  }
}

fn add_ancestor_states_to_enter(
  state_map: &OrderedMap<&'static str, StateNode>,
  state_id: &'static str,
  ancestor_id: &'static str,
  states_to_enter: &mut Vec<&'static str>,
  states_for_default_entry: &mut Vec<&'static str>,
  default_history_actions: &mut HashMap<&'static str, &'static [&'static Action]>,
) {
  for ancestor_id in utils::get_proper_ancestor_ids(state_id, Some(ancestor_id)) {
    if let Some(ancestor) = state_map.get(ancestor_id) {
      states_to_enter.push(ancestor_id);

      match ancestor {
        StateNode::Parallel(_) => {
          for &child_id in ancestor.child_state_ids() {
            if !states_to_enter
              .iter()
              .any(|&s| utils::is_descendant(s, child_id))
            {
              add_descendent_states_to_enter(
                state_map,
                child_id,
                states_to_enter,
                states_for_default_entry,
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
                .all(|&s| utils::is_descendant(s, transition.source))
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
  for &ancestor_id in utils::get_proper_ancestor_ids(state_list[0], None)
    .iter()
    .filter(|&&state_id| {
      if let Some(state) = state_map.get(state_id) {
        match state {
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
      .all(|&s| utils::is_descendant(s, ancestor_id))
    {
      return Some(ancestor_id);
    }
  }

  None
}
