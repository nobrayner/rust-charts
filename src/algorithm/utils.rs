use std::collections::HashMap;

use crate::{event::Event, transition::Transition};

pub fn is_descendant(state_1: &str, state_2: &str) -> bool {
  state_1.starts_with(state_2)
}

pub fn condition_match(transition: &Transition) -> bool {
  match transition.cond {
    Some(cond) => cond(
      // FIXME: Use real event and context here?
      Event {
        name: String::from(""),
        data: HashMap::new(),
      },
    ),
    None => true,
  }
}

pub fn get_proper_ancestor_ids<'s>(
  state_id: &'s str,
  maybe_ancestor_id: Option<&'s str>,
) -> Vec<&'s str> {
  let mut ancestors = vec![];
  let ancestor_id = match maybe_ancestor_id {
    Some(id) => id,
    None => "",
  };

  if state_id == ancestor_id || ancestor_id.starts_with(state_id) {
    return ancestors;
  }

  if let Some(last_dot_index) = (&state_id).rfind(".") {
    let mut current_index = last_dot_index;
    loop {
      let id = &state_id[..current_index];

      if id == ancestor_id {
        break;
      }

      if let Some(next_last_dot_index) = id.rfind(".") {
        ancestors.push(id);
        current_index = next_last_dot_index;
      } else {
        // We've reached the root
        ancestors.push(id);
        break;
      }
    }
  }

  ancestors
}
