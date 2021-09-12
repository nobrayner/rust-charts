use phf;
use std::{collections::HashMap, fmt};

use crate::{algorithm, event::Event, state::State, state_node::State as StateNode, Transition};

pub struct Machine {
  pub id: &'static str,
  pub initial: Transition,
  pub states: phf::OrderedMap<&'static str, StateNode>,
}
impl Machine {
  pub fn initial_state(&self) -> State {
    algorithm::initial_state(&self.states, &self.initial)
  }

  pub fn transition(&self, state: State, event: &str) -> State {
    algorithm::event_loop_step(
      &self.states,
      state,
      Event {
        name: String::from(event),
        data: HashMap::new(),
      },
    )
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
    let states = &self.states;
    let states_vec = states.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    f.debug_struct("Machine")
      .field("id", &self.id)
      .field("root", &self.initial)
      .field("states", &states_vec)
      // .field("actions", &self.actions.len())
      .finish()
  }
}
