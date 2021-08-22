use std::{collections::HashMap, fmt};

#[derive(Clone)]
pub struct Action {
  pub(crate) kind: String,
  pub(crate) exec: fn() -> (),
  pub(crate) data: HashMap<String, String>, // <String, Any>, but not sure how that will go
}
impl fmt::Debug for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Action").field("type", &self.kind).finish()
  }
}
impl Action {
  pub fn stub() -> Self {
    Self {
      kind: String::from(""),
      exec: || (),
      data: HashMap::new(),
    }
  }
}
