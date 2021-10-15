use crate::{
  event::Event, machine::MachineMappings, state_node::StateNode, transition::Transition,
  StateNodeTrait,
};

pub fn is_descendant(mappings: &MachineMappings, child_id: &str, ancestor_id: &str) -> bool {
  let mut is_descendant = false;

  let mut marker = mappings.state(child_id);

  while let Some(parent_id) = marker.parent() {
    if parent_id != ancestor_id {
      marker = mappings.state(parent_id);
    } else {
      is_descendant = true;
      break;
    }
  }

  is_descendant
}

pub fn guard_match(mappings: &MachineMappings, transition: &Transition, event: &Event) -> bool {
  match transition.guard_id() {
    Some(guard_id) => mappings.guard(guard_id)(event),
    None => true,
  }
}

pub fn get_proper_ancestor_ids<'s>(
  mappings: &'s MachineMappings,
  state_id: &'s str,
  maybe_ancestor_id: Option<&'s str>,
) -> Vec<&'s String> {
  let mut ancestors = vec![];
  let mut ancestor_found = maybe_ancestor_id.is_none();
  let ancestor_id = match maybe_ancestor_id {
    Some(id) => &String::from(id),
    None => &String::from(""),
  };

  if state_id == ancestor_id {
    return ancestors;
  }

  let mut marker = mappings.state(state_id).parent();

  while let Some(parent_id) = marker {
    if parent_id != ancestor_id {
      ancestors.push(parent_id);
      marker = mappings.state(parent_id).parent();
    } else {
      ancestor_found = true;
      break;
    }
  }

  if ancestor_found {
    ancestors
  } else {
    vec![]
  }
}

pub fn is_in_final_state(
  mappings: &MachineMappings,
  configuration: &[String],
  state_id: &str,
) -> bool {
  match mappings.state(state_id) {
    StateNode::Compound(state) => {
      state
        .child_state_ids()
        .into_iter()
        .any(|child_id| match mappings.state(child_id) {
          StateNode::Final(_) => configuration.contains(child_id),
          _ => false,
        })
    }
    StateNode::Parallel(state) => state
      .child_state_ids()
      .into_iter()
      .all(|child_id| is_in_final_state(mappings, configuration, child_id)),
    _ => false,
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::{
//     map,
//     state_node::{AtomicStateNode, CompoundStateNode, StateNode},
//     RootStateNode, SCXML_ROOT_ID,
//   };

//   static STATE_MAP: OrderedMap<&'static str, StateNode> = {
//     let scxml_root = StateNode::Root(RootStateNode {});
//     let grandparent = StateNode::Compound(CompoundStateNode {
//       id: "grandparent",
//       parent: SCXML_ROOT_ID,
//       always: &[],
//       on: map! {},
//       initial: None,
//       states: &["grandparent.parent"],
//       history_states: &[],
//       entry: &[],
//       exit: &[],
//     });
//     let grandparent_parent = StateNode::Compound(CompoundStateNode {
//       id: "grandparent.parent",
//       parent: "grandparent",
//       always: &[],
//       on: map! {},
//       initial: None,
//       states: &["grandparent.parent.child"],
//       history_states: &[],
//       entry: &[],
//       exit: &[],
//     });
//     let grandparent_parent_child = StateNode::Atomic(AtomicStateNode {
//       id: "grandparent.parent.child",
//       parent: "grandparent.parent",
//       always: &[],
//       on: map! {},
//       entry: &[],
//       exit: &[],
//     });
//     let orphan = StateNode::Atomic(AtomicStateNode {
//       id: "orphan",
//       parent: SCXML_ROOT_ID,
//       always: &[],
//       on: map! {},
//       entry: &[],
//       exit: &[],
//     });

//     map! {
//       "scxml::root" => scxml_root,
//       "grandparent" => grandparent,
//       "grandparent.parent" => grandparent_parent,
//       "grandparent.parent.child" => grandparent_parent_child,
//       "orphan" => orphan,
//     }
//   };

//   #[test]
//   fn test_is_descendant() {
//     assert_eq!(
//       is_descendant(&STATE_MAP, "grandparent.parent", "grandparent"),
//       true,
//       "grandparent > grandparent.parent"
//     );

//     assert_eq!(
//       is_descendant(&STATE_MAP, "grandparent.parent.child", "grandparent"),
//       true,
//       "grandparent > grandparent.parent.child"
//     );

//     assert_eq!(
//       is_descendant(&STATE_MAP, "orphan", "grandparent"),
//       false,
//       "grandparent !> orphan"
//     );

//     assert_eq!(
//       is_descendant(&STATE_MAP, "grandparent", "grandparent.parent"),
//       false,
//       "grandparent.parent !> grandparent"
//     );
//   }

//   #[test]
//   fn test_get_proper_ancestor_ids() {
//     assert_eq!(
//       get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent.child", Some("grandparent")),
//       vec!["grandparent.parent"]
//     );

//     assert_eq!(
//       get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent.child", None),
//       // NOTE: This also validates the returned vec is in ancestry order (walking up the tree)
//       vec!["grandparent.parent", "grandparent", SCXML_ROOT_ID]
//     );

//     assert_eq!(
//       get_proper_ancestor_ids(&STATE_MAP, "grandparent.parent", Some("grandparent")),
//       vec![] as Vec<&str>,
//     );

//     assert_eq!(
//       get_proper_ancestor_ids(&STATE_MAP, "grandparent", Some("grandparent")),
//       vec![] as Vec<&str>,
//       "same node should return empty"
//     );

//     assert_eq!(
//       get_proper_ancestor_ids(&STATE_MAP, "grandparent", Some("grandparent.parent")),
//       vec![] as Vec<&str>,
//       "descendant should return empty"
//     );
//   }
// }
