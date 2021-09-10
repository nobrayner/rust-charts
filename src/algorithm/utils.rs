use phf::OrderedMap;
use std::collections::HashMap;

use crate::{event::Event, state_node::State as StateNode, transition::Transition};

pub fn is_descendant(
  state_map: &OrderedMap<&'static str, StateNode>,
  child_id: &str,
  ancestor_id: &str,
) -> bool {
  let mut is_descendant = false;

  if let Some(child) = state_map.get(child_id) {
    let mut marker = child;

    while let Some(parent_id) = marker.parent() {
      if parent_id != ancestor_id {
        if let Some(parent) = state_map.get(parent_id) {
          marker = parent;
        } else {
          // Technically shouldn't be possible
          panic!("Invalid state \"{}\"", parent_id);
        }
      } else {
        is_descendant = true;
        break;
      }
    }
  }

  is_descendant
}

pub fn guard_match(transition: &Transition) -> bool {
  match transition.guard {
    Some(guard) => guard(
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
  state_map: &OrderedMap<&'static str, StateNode>,
  state_id: &'s str,
  maybe_ancestor_id: Option<&'s str>,
) -> Vec<&'s str> {
  let mut ancestors = vec![];
  let ancestor_id = match maybe_ancestor_id {
    Some(id) => id,
    None => "",
  };

  if let Some(state) = state_map.get(state_id) {
    let mut marker = state.parent();

    while let Some(parent_id) = marker {
      if parent_id != ancestor_id {
        if let Some(parent) = state_map.get(parent_id) {
          ancestors.push(parent_id);
          marker = parent.parent();
        } else {
          // Technically shouldn't be possible
          panic!("Invalid state \"{}\"", parent_id);
        }
      } else {
        break;
      }
    }
  }

  ancestors
}

pub fn is_in_final_state(
  state_map: &OrderedMap<&'static str, StateNode>,
  configuration: &[&'static str],
  state_id: &'static str,
) -> bool {
  if let Some(state) = state_map.get(state_id) {
    match state {
      StateNode::Compound(_) => state.child_state_ids().into_iter().any(|child_id| {
        is_in_final_state(state_map, configuration, child_id) && configuration.contains(child_id)
      }),
      StateNode::Parallel(_) => state
        .child_state_ids()
        .into_iter()
        .all(|child_id| is_in_final_state(state_map, configuration, child_id)),
      _ => false,
    }
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::state_node::{AtomicStateNode, CompoundStateNode, State as StateNode};
  use phf::phf_ordered_map;

  static STATE_MAP: OrderedMap<&'static str, StateNode> = {
    let grandparent = StateNode::Compound(CompoundStateNode {
      id: "grandparent",
      parent: None,
      initial: None,
      on: phf_ordered_map! {},
      states: &["grandparent.parent"],
    });
    let grandparent_parent = StateNode::Compound(CompoundStateNode {
      id: "grandparent.parent",
      parent: Some("grandparent"),
      initial: None,
      on: phf_ordered_map! {},
      states: &["grandparent.parent.child"],
    });
    let grandparent_parent_child = StateNode::Atomic(AtomicStateNode {
      id: "grandparent.parent.child",
      parent: Some("grandparent.parent"),
      on: phf_ordered_map! {},
    });
    let orphan = StateNode::Atomic(AtomicStateNode {
      id: "orphan",
      parent: None,
      on: phf_ordered_map! {},
    });

    phf_ordered_map! {
      "grandparent" => grandparent,
      "grandparent.parent" => grandparent_parent,
      "grandparent.parent.child" => grandparent_parent_child,
      "orphan" => orphan,
    }
  };

  #[test]
  fn test_is_descendant() {
    assert_eq!(
      is_descendant(&STATE_MAP, "grandparent.parent", "grandparent"),
      true,
      "grandparent > grandparent.parent"
    );

    assert_eq!(
      is_descendant(&STATE_MAP, "grandparent.parent.child", "grandparent"),
      true,
      "grandparent > grandparent.parent.child"
    );

    assert_eq!(
      is_descendant(&STATE_MAP, "orphan", "grandparent"),
      false,
      "grandparent !> orphan"
    );

    assert_eq!(
      is_descendant(&STATE_MAP, "grandparent", "grandparent.parent"),
      false,
      "grandparent.parent !> grandparent"
    );
  }

  #[test]
  fn test_get_proper_ancestor_ids() {
    assert_eq!(
      get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent.child", Some("grandparent")),
      vec!["grandparent.parent"]
    );

    assert_eq!(
      get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent.child", None),
      vec!["grandparent.parent", "grandparent"]
    );

    assert_eq!(
      get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent", Some("grandparent")),
      vec![] as Vec<&str>
    );

    assert_eq!(
      get_proper_ancestor_ids(&STATE_MAP, "grandparent", Some("grandparent")),
      vec![] as Vec<&str>
    );

    assert_eq!(
      get_proper_ancestor_ids(&STATE_MAP, "grandparent", Some("grandparent.parent")),
      vec![] as Vec<&str>
    );
  }
}
