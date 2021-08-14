use std::collections::HashMap;

#[derive(Debug)]
pub struct Event {
  pub(crate) name: String,
  pub(crate) data: HashMap<String, String>,
}
impl Event {
  pub fn new() -> Self {
    Event {
      name: String::from(""),
      data: HashMap::new(),
    }
  }
}
