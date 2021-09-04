// add_descendent_states_to_enter
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
//   default_history_content[state.parent.id] = state.transition.content;
//   if let Some(target_transition) = self.initial() {
//     for target_state_id in target_transition.target() {
//       if let Some(target_state) = state_map.get(target_state_id) {
//         target_state.add_descendent_states_to_enter(
//           state_map,
//           states_to_enter,
//           states_for_default_entry,
//         );
//         if let Some(target_parent_id) = target_state.parent() {
//           if let Some(target_parent_state) = state_map.get(target_parent_id) {
//             target_parent_state.add_ancestor_states_to_enter(
//               s,
//               ancestor=s.parent,
//               states_to_enter=states_to_enter,
//               states_for_default_entry=states_for_default_entry,
//               default_history_content=default_history_content,
//               history_value=history_value,
//             );
//           }
//         }
//       }
//     }
//   }
// }

// fn get_internal_transition_domain(&self, _: Vec<&str>, _: &'static str) -> Option<&'static str> {
//   None
// }

// get_effective_target_state_ids
// FIXME:
// panic!("History states not supported yet!");
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
