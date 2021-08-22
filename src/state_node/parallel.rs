// add_descendent_states_to_enter
// states_to_enter.insert(state_id.clone());
// for child in get_child_states(state_map, &state_id) {
//   if states_to_enter.iter().any(|s| is_descendant(s, &child)) {
//     add_descendent_states_to_enter(
//       state_map,
//       child,
//       states_to_enter,
//       states_for_default_entry,
//       default_history_content,
//       history_value,
//     )
//   }
// }

// add_ancestor_states_to_enter
// for anc in get_proper_ancestors(state_id, ancestor_id) {
//   states_to_enter.insert(anc.clone());
//   for child in get_child_states(state_map, &anc) {
//     if !(states_to_enter.iter().any(|s| is_descendant(s, &child))) {
//       add_descendent_states_to_enter(
//         state_map,
//         child,
//         states_to_enter,
//         states_for_default_entry,
//         default_history_content,
//         history_value,
//       );
//     }
//   }
// }

// is_in_final_state
// self.child_states()
//   .iter()
//   .all(|child_id| {
//     if let Some(child) = state_map.get(child_id) {
//       child.is_in_final_state(configuration)
//     } else {
//       false
//     }
//   })

// fn get_internal_transition_domain(&self, _: Vec<&str>, _: &'static str) -> Option<&'static str> {
//   None
// }
