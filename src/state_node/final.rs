// add_descendent_states_to_enter
// states_to_enter.insert(self.id());

// is_in_final_state
// configuration.contains(self.id)

// fn get_internal_transition_domain(&self, _: Vec<&str>, _: &'static str) -> Option<&'static str> {
//   None
// }

// fn get_effective_target_state_ids(&self, target_ids: &mut HashSet<&'static str>) {
//   target_ids.insert(self.id);
// }

// enter_states
// if state.is_final() {
//   if let Some(parent_id) = state.parent() {
//     internal_queue.push(Event {
//       name: String::from("done.state.") + parent_id,
//       // data: match state.done_data.clone() {
//       //   Some(d) => d,
//       //   None => HashMap::new(),
//       // },
//       data: HashMap::new(),
//     });

//     if let Some(parent) = state_map.get(parent_id) {
//       if let Some(grandparent_id) = parent.parent() {
//         if is_parallel_state(state_map, &grandparent_id) {
//           if state
//             .child_states()
//             .iter()
//             .all(|parent_state_id| parent.is_in_final_state(state_map, &configuration))
//           {
//             internal_queue.push(Event {
//               name: String::from("done.state.") + grandparent_id,
//               data: HashMap::new(),
//             })
//           }
//         }
//       }
//     }
//   }
// }
