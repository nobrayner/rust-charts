use std::collections::{HashMap, VecDeque};

use crate::{
  event::Event,
  machine::MachineMappings,
  state::State,
  state_node::{HistoryKind, StateNode},
  transition::{Transition, TransitionKind},
  StateNodeTrait,
};

pub mod utils;

pub fn initial_state(mappings: MachineMappings, initial_transition: &Transition) -> State {
  let mut internal_queue = VecDeque::new();
  // let mut states_to_invoke = vec![];
  let transitions = vec![initial_transition];

  let mut initial_state = State {
    configuration: vec![],
    actions: vec![],
    history: HashMap::new(),
  };

  enter_states(
    &mappings,
    &transitions,
    &mut internal_queue,
    &mut initial_state,
  );

  // Step 1: Macro step
  macrostep(
    &mappings,
    &Event {
      name: String::from("rust_charts::init"),
      data: HashMap::new(),
    },
    &mut internal_queue,
    &mut initial_state,
  );

  // Step 2: Invoke
  // TODO: Invoke states_to_invoke

  // Step 3: Repeat step 1 for any invoke errors
  // TODO: perform another event loop step if there are errors?
  // if !internal_queue.is_empty() { macrostep(); }

  initial_state
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
  mappings: MachineMappings,
  mut current_state: State,
  triggered_event: Event,
) -> State {
  let mut internal_queue = VecDeque::new();

  // Reset actions queue
  current_state.actions = vec![];

  // Step 4: Process external event
  // TODO: Cancel event performs any cleanup that needs to occur
  // if is_cancel_event(event) { event_loop_step(state, event); }

  // Invoke anything based on the event

  let enabled_transitions = select_transitions(&mappings, &triggered_event, &mut current_state);

  if !enabled_transitions.is_empty() {
    microstep(
      &mappings,
      &enabled_transitions,
      &mut internal_queue,
      &mut current_state,
    );
  }

  // Step 1: Macro step
  macrostep(
    &mappings,
    // TODO: Is this correct?
    &triggered_event,
    &mut internal_queue,
    &mut current_state,
  );

  // Step 2: Invoke
  // TODO: Invoke states_to_invoke

  // Step 3: Repeat step 1 for any invoke errors
  // TODO: perform another event loop step if there are errors?
  // if !internal_queue.is_empty() { macrostep(); }

  current_state
}

fn macrostep(
  mappings: &MachineMappings,
  event: &Event,
  internal_queue: &mut VecDeque<Event>,
  current_state: &mut State,
) {
  let mut enabled_transitions;

  let mut done = false;

  while done == false {
    enabled_transitions = select_eventless_transitions(mappings, event, current_state);

    if enabled_transitions.is_empty() {
      if internal_queue.is_empty() {
        done = true;
      } else {
        let maybe_event = internal_queue.pop_front();
        if let Some(internal_event) = maybe_event {
          enabled_transitions = select_transitions(mappings, &internal_event, current_state);
        }
      }
    }
    if !enabled_transitions.is_empty() {
      microstep(
        mappings,
        &enabled_transitions,
        internal_queue,
        current_state,
      );
    }
  }
}

fn microstep(
  mappings: &MachineMappings,
  enabled_transitions: &[&Transition],
  internal_queue: &mut VecDeque<Event>,
  current_state: &mut State,
) {
  exit_states(mappings, enabled_transitions, current_state);

  // TODO: Accumulate transition actions
  enabled_transitions.iter().for_each(|&t| {
    for action_id in t.actions() {
      current_state.add_action(action_id)
    }
  });

  enter_states(mappings, enabled_transitions, internal_queue, current_state);
}

fn select_eventless_transitions<'s>(
  mappings: &'s MachineMappings,
  event: &Event,
  current_state: &mut State,
) -> Vec<&'s Transition> {
  let mut enabled_transitions = vec![];
  // TODO: document order sort?
  let atomic_states: Vec<_> = current_state
    .configuration
    .iter()
    .filter(|&state_id| match mappings.state(state_id) {
      StateNode::Atomic(_) => true,
      StateNode::Final(_) => true,
      _ => false,
    })
    .collect();

  for atomic_state_id in atomic_states {
    let mut state_and_ancestor_ids = vec![atomic_state_id];
    state_and_ancestor_ids.append(&mut utils::get_proper_ancestor_ids(
      mappings,
      atomic_state_id,
      None,
    ));

    let mut looping = true;
    for state_id in state_and_ancestor_ids {
      if looping == false {
        break;
      }

      for transition in mappings.state(&state_id).eventless_transitions() {
        if utils::guard_match(mappings, transition, event) {
          enabled_transitions.push(transition);
          looping = false;
        }
      }
    }
  }

  remove_conflicting_transitions(mappings, enabled_transitions, current_state)
}

fn select_transitions<'s>(
  mappings: &'s MachineMappings,
  event: &Event,
  current_state: &mut State,
) -> Vec<&'s Transition> {
  let mut enabled_transitions = vec![];
  // TODO: document order sort?
  let atomic_states: Vec<_> = current_state
    .configuration
    .iter()
    .filter(|&state_id| match mappings.state(state_id) {
      StateNode::Atomic(_) => true,
      StateNode::Final(_) => true,
      _ => false,
    })
    .collect();

  for atomic_state_id in atomic_states {
    let mut state_and_ancestor_ids = vec![atomic_state_id];
    state_and_ancestor_ids.append(&mut utils::get_proper_ancestor_ids(
      mappings,
      atomic_state_id,
      None,
    ));

    let mut looping = true;
    for state_id in state_and_ancestor_ids {
      if looping == false {
        break;
      }

      let state = mappings.state(state_id);
      for transition in state.on(&event.name) {
        if utils::guard_match(mappings, transition, event) {
          enabled_transitions.push(transition);
          looping = false;
        }
      }
    }
  }

  remove_conflicting_transitions(mappings, enabled_transitions, current_state)
}

fn remove_conflicting_transitions<'s>(
  mappings: &MachineMappings,
  enabled_transitions: Vec<&'s Transition>,
  current_state: &mut State,
) -> Vec<&'s Transition> {
  let mut filtered_transitions = vec![];

  for t1 in enabled_transitions {
    let mut t1_preempted = false;
    let mut transitions_to_remove = vec![];

    for &t2 in &filtered_transitions {
      let t1_exit_set = compute_exit_set(mappings, &vec![t1], current_state);
      let t2_exit_set = compute_exit_set(mappings, &vec![t2], current_state);

      let has_intersection = t1_exit_set.iter().any(|t| t2_exit_set.contains(t))
        || t2_exit_set.iter().any(|t| t1_exit_set.contains(t));

      if has_intersection {
        if utils::is_descendant(mappings, t1.source(), t2.source()) {
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
  mappings: &MachineMappings,
  enabled_transitions: &[&Transition],
  current_state: &mut State,
) {
  let state_ids_to_exit = compute_exit_set(mappings, enabled_transitions, current_state);

  for _state_id in &state_ids_to_exit {
    // TODO: states_to_invoke
    // states_to_invoke.remove(states_to_invoke.iter().position(|&s| s == _state_id));
  }

  // TODO: Sort by `exit_order`
  // state_ids_to_exit = state_ids_to_exit.sort_by(exit_order);

  for state_id in &state_ids_to_exit {
    let state = mappings.state(state_id);
    for history_id in state.history_state_ids() {
      if let StateNode::History(history) = mappings.state(history_id) {
        let history_configuration =
          current_state
            .configuration
            .iter()
            .fold(vec![], |mut history_config, id| {
              let state = mappings.state(id);
              match history.kind {
                HistoryKind::Deep => match state {
                  StateNode::Atomic(_) => {
                    if utils::is_descendant(mappings, state.id(), state_id) {
                      history_config.push(id.clone());
                    }
                  }
                  _ => (),
                },
                HistoryKind::Shallow => {
                  if let Some(parent) = state.parent() {
                    if parent == state_id {
                      history_config.push(id.clone());
                    }
                  }
                }
              }

              history_config
            });
        current_state.update_history(history_id, history_configuration);
      }
    }
  }
  for state_id in &state_ids_to_exit {
    let state = mappings.state(state_id);

    for action_id in state.exit_actions() {
      current_state.add_action(action_id);
    }

    // TODO: Invoking stuff
    // for inv in state.invoke() {
    //   cancel_invoke(inv);
    // }

    current_state.remove_configuration(state_id);
  }
}

fn compute_exit_set<'s>(
  mappings: &MachineMappings,
  enabled_transitions: &[&Transition],
  current_state: &mut State,
) -> Vec<String> {
  let mut state_ids_to_exit = vec![];

  for &transition in enabled_transitions {
    if !transition.targets().is_empty() {
      let maybe_domain = get_transition_domain(mappings, transition, current_state);

      if let Some(domain) = maybe_domain {
        for state_id in &current_state.configuration {
          if utils::is_descendant(mappings, state_id, &domain) {
            state_ids_to_exit.push(state_id.clone());
          }
        }
      }
    }
  }

  state_ids_to_exit
}

fn enter_states(
  mappings: &MachineMappings,
  enabled_transitions: &[&Transition],
  internal_queue: &mut VecDeque<Event>,
  current_state: &mut State,
) {
  let (state_ids_to_enter, state_ids_for_default_entry, default_history_actions) =
    compute_entry_set(mappings, enabled_transitions, current_state);

  // TODO: Sort by `entry_order`
  for state_id in state_ids_to_enter {
    let state = mappings.state(&state_id);
    current_state.add_configuration(state_id.clone());

    // TODO: states_to_invoke
    // states_to_invoke.push(state_id);

    for action_id in state.entry_actions() {
      current_state.add_action(action_id);
    }

    if state_ids_for_default_entry.contains(&state_id) {
      if let Some(transition) = state.initial() {
        for action_id in transition.actions() {
          current_state.add_action(action_id);
        }
      }
    }

    if let Some(&actions) = default_history_actions.get(&state_id) {
      for action_id in actions {
        current_state.add_action(action_id);
      }
    }

    match state {
      StateNode::Final(_) => {
        if let Some(parent_id) = state.parent() {
          internal_queue.push_back(Event {
            name: String::from("done.state.") + parent_id,
            data: HashMap::new(),
          });

          if let Some(grandparent_id) = mappings.state(parent_id).parent() {
            let grandparent = mappings.state(grandparent_id);
            match grandparent {
              StateNode::Parallel(_) => {
                if grandparent
                  .child_state_ids()
                  .into_iter()
                  .all(|child_state_id| {
                    utils::is_in_final_state(mappings, &current_state.configuration, child_state_id)
                  })
                {
                  internal_queue.push_back(Event {
                    name: String::from("done.state.") + grandparent_id,
                    data: HashMap::new(),
                  })
                }
              }
              _ => (),
            }
          }
        }
      }
      _ => (),
    };
  }
}

fn compute_entry_set<'s>(
  mappings: &'s MachineMappings,
  enabled_transitions: &'s [&Transition],
  current_state: &State,
) -> (Vec<String>, Vec<String>, HashMap<String, &'s [String]>) {
  let mut state_ids_to_enter = vec![];
  let mut state_ids_for_default_entry = vec![];
  let mut default_history_actions = HashMap::new();

  for &transition in enabled_transitions {
    for state_id in transition.targets() {
      add_descendent_states_to_enter(
        mappings,
        state_id.clone(),
        &mut state_ids_to_enter,
        &mut state_ids_for_default_entry,
        &mut default_history_actions,
        current_state,
      );
    }

    let maybe_ancestor_id = get_transition_domain(mappings, transition, current_state);

    if let Some(ancestor_id) = maybe_ancestor_id {
      for state_id in get_effective_target_states(mappings, transition, current_state) {
        add_ancestor_states_to_enter(
          mappings,
          state_id,
          ancestor_id.clone(),
          &mut state_ids_to_enter,
          &mut state_ids_for_default_entry,
          &mut default_history_actions,
          current_state,
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

fn add_descendent_states_to_enter<'s>(
  mappings: &'s MachineMappings,
  state_id: String,
  state_ids_to_enter: &mut Vec<String>,
  state_ids_for_default_entry: &mut Vec<String>,
  default_history_actions: &mut HashMap<String, &'s [String]>,
  current_state: &State,
) {
  match mappings.state(&state_id) {
    StateNode::History(state) => {
      if let Some(history_state_ids) = current_state.history.get(&state_id) {
        for history_state_id in history_state_ids {
          add_descendent_states_to_enter(
            mappings,
            history_state_id.clone(),
            state_ids_to_enter,
            state_ids_for_default_entry,
            default_history_actions,
            current_state,
          );
        }
        if let Some(parent_id) = state.parent() {
          for history_state_id in history_state_ids {
            add_ancestor_states_to_enter(
              mappings,
              history_state_id.clone(),
              parent_id.clone(),
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
              current_state,
            )
          }
        }
      } else {
        // History states have one transition that is required
        let transition = state.target();

        for target_state_id in transition.targets() {
          add_descendent_states_to_enter(
            mappings,
            target_state_id.clone(),
            state_ids_to_enter,
            state_ids_for_default_entry,
            default_history_actions,
            current_state,
          );
        }

        if let Some(parent_id) = state.parent() {
          default_history_actions.insert(String::from(parent_id), transition.actions());

          for target_state_id in transition.targets() {
            add_ancestor_states_to_enter(
              mappings,
              target_state_id.clone(),
              parent_id.clone(),
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
              current_state,
            );
          }
        }
      }
    }
    StateNode::Compound(state) => {
      state_ids_to_enter.push(state_id.clone());
      state_ids_for_default_entry.push(state_id.clone());

      if let Some(transition) = state.initial() {
        for target_state_id in transition.targets() {
          add_descendent_states_to_enter(
            mappings,
            target_state_id.clone(),
            state_ids_to_enter,
            state_ids_for_default_entry,
            default_history_actions,
            current_state,
          );
        }

        for target_state_id in transition.targets() {
          add_ancestor_states_to_enter(
            mappings,
            target_state_id.clone(),
            state_id.clone(),
            state_ids_to_enter,
            state_ids_for_default_entry,
            default_history_actions,
            current_state,
          );
        }
      }
    }
    StateNode::Parallel(state) => {
      state_ids_to_enter.push(String::from(state_id));

      for child_id in state.child_state_ids() {
        if !state_ids_to_enter
          .iter()
          .any(|s| utils::is_descendant(mappings, s, child_id))
        {
          add_descendent_states_to_enter(
            mappings,
            child_id.clone(),
            state_ids_to_enter,
            state_ids_for_default_entry,
            default_history_actions,
            current_state,
          );
        }
      }
    }
    _ => state_ids_to_enter.push(String::from(state_id)),
  }
}

fn add_ancestor_states_to_enter<'s>(
  mappings: &'s MachineMappings,
  state_id: String,
  ancestor_id: String,
  state_ids_to_enter: &mut Vec<String>,
  state_ids_for_default_entry: &mut Vec<String>,
  default_history_actions: &mut HashMap<String, &'s [String]>,
  current_state: &State,
) {
  for ancestor_id in utils::get_proper_ancestor_ids(mappings, &state_id, Some(&ancestor_id)) {
    state_ids_to_enter.push(ancestor_id.clone());

    match mappings.state(&ancestor_id) {
      StateNode::Parallel(ancestor) => {
        for child_id in ancestor.child_state_ids() {
          if !state_ids_to_enter
            .iter()
            .any(|s| utils::is_descendant(mappings, s, child_id))
          {
            add_descendent_states_to_enter(
              mappings,
              child_id.clone(),
              state_ids_to_enter,
              state_ids_for_default_entry,
              default_history_actions,
              current_state,
            );
          }
        }
      }
      _ => (),
    }
  }
}

fn get_transition_domain<'s>(
  mappings: &'s MachineMappings,
  transition: &'s Transition,
  current_state: &State,
) -> Option<String> {
  let transition_state_ids = get_effective_target_states(mappings, transition, current_state);

  let lcca = |mut transition_state_ids| {
    let mut state_list = vec![transition.source().clone()];
    state_list.append(&mut transition_state_ids);

    find_lcca(mappings, state_list)
  };

  if transition_state_ids.is_empty() {
    None
  } else {
    match transition.kind() {
      TransitionKind::Internal => match mappings.state(transition.source()) {
        StateNode::Compound(_) => {
          if transition_state_ids
            .iter()
            .all(|s| utils::is_descendant(mappings, s, transition.source()))
          {
            Some(transition.source().clone())
          } else {
            lcca(transition_state_ids)
          }
        }
        _ => lcca(transition_state_ids),
      },
      _ => lcca(transition_state_ids),
    }
  }
}

fn get_effective_target_states<'s>(
  mappings: &'s MachineMappings,
  transition: &'s Transition,
  current_state: &State,
) -> Vec<String> {
  let mut targets = vec![];

  for target_state_id in transition.targets() {
    match mappings.state(target_state_id) {
      StateNode::History(target_state) => {
        if let Some(history_state_ids) = current_state.history.get(target_state_id) {
          history_state_ids
            .iter()
            .for_each(|state_id| targets.push(state_id.clone()));
        } else {
          get_effective_target_states(mappings, target_state.target(), current_state)
            .iter()
            .for_each(|state_id| targets.push(state_id.clone()));
        }
      }
      _ => targets.push(target_state_id.clone()),
    }
  }

  targets
}

fn find_lcca<'s>(state_map: &'s MachineMappings, state_list: Vec<String>) -> Option<String> {
  for &ancestor_id in utils::get_proper_ancestor_ids(state_map, &state_list[0], None)
    .iter()
    .filter(|&state_id| {
      match state_map.state(state_id) {
        // The root node also counts as an LCCA
        StateNode::Root(_) => true,
        StateNode::Compound(_) => true,
        _ => false,
      }
    })
  {
    if state_list[1..]
      .iter()
      .all(|s| utils::is_descendant(state_map, &s, ancestor_id))
    {
      return Some(ancestor_id.clone());
    }
  }

  // Technically should never happen, as all state nodes have the scxml root
  None
}
