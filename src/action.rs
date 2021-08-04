use std::{collections::HashMap, fmt};

pub struct Action {
  kind: String,
  exec: Box<dyn Fn() -> ()>,
  data: HashMap<String, String>, // <String, Any>, but not sure how that will go
}
impl fmt::Debug for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Action").field("type", &self.kind).finish()
  }
}
impl Action {
  pub fn new() -> Self {
    Self {
      kind: String::from(""),
      exec: Box::new(|| ()),
      data: HashMap::new(),
    }
  }
}
