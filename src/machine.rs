use std::{collections::HashMap, fmt};

use crate::{
  action::Action, algorithm, event::Event, state::State, state_node::StateNode, types::GuardFn,
  Transition, SCXML_ROOT_ID,
};

pub struct Machine {
  id: String,
  initial: Transition,
  states: HashMap<String, StateNode>,
  actions: HashMap<String, Action>,
  guards: HashMap<String, GuardFn>,
}
impl Machine {
  pub fn new(
    id: &str,
    initial: String,
    states: HashMap<String, StateNode>,
    actions: HashMap<String, Action>,
    guards: HashMap<String, GuardFn>,
  ) -> Self {
    Self {
      id: String::from(id),
      initial: Transition {
        targets: vec![initial],
        actions: vec![],
        guard: None,
        kind: crate::TransitionKind::Internal,
        source: SCXML_ROOT_ID,
      },
      states,
      actions,
      guards,
    }
  }

  pub fn initial_state(&self) -> State {
    algorithm::initial_state(self.mappings(), &self.initial)
  }

  pub fn transition(&self, state: State, event: &str) -> State {
    algorithm::event_loop_step(
      self.mappings(),
      state,
      Event {
        name: String::from(event),
        data: HashMap::new(),
      },
    )
  }

  fn mappings(&self) -> MachineMappings {
    MachineMappings {
      states: &self.states,
      actions: &self.actions,
      guards: &self.guards,
    }
  }

  // pub fn state_from(&self, state_values: Vec<&'static str>) -> State {
  //   State {
  //     value: self.get_state_values(state_values, None),
  //     // configuration: vec![],
  //     actions: vec![],
  //   }
  // }

  // TODO: machine.state_from(["state_id"]);
  // fn get_state_values(
  //   &self,
  //   state_values: Vec<&'static str>,
  //   parent: Option<&'static str>,
  // ) -> Vec<&'static str> {
  //   let states: HashSet<_> = state_values
  //     .into_iter()
  //     .map(|potential_state| {
  //       let index = match parent {
  //         Some(parent) => String::from(parent) + "." + potential_state,
  //         None => String::from(potential_state),
  //       };
  //       if let Some(_) = self.states.get(&index) {
  //         return potential_state;
  //       } else {
  //         panic!("State node {} is missing", potential_state);
  //       }
  //     })
  //     .collect();

  //   Vec::from_iter(states)
  // }
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.initial)
      .field("states", &self.states)
      .field("actions", &self.actions)
      .finish()
  }
}

pub struct MachineMappings<'m> {
  states: &'m HashMap<String, StateNode>,
  actions: &'m HashMap<String, Action>,
  guards: &'m HashMap<String, GuardFn>,
}
impl<'m> MachineMappings<'m> {
  pub fn state(&self, state_id: &str) -> &StateNode {
    self
      .states
      .get(state_id)
      .expect(&format!("Invalid state id: {}", state_id))
  }

  pub fn action(&self, action_id: &str) -> &Action {
    self
      .actions
      .get(action_id)
      .expect(&format!("Invalid action id: {}", action_id))
  }

  pub fn guard(&self, guard_id: &str) -> &GuardFn {
    self
      .guards
      .get(guard_id)
      .expect(&format!("Invalid guard id: {}", guard_id))
  }
}
