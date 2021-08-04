use std::collections::HashMap;

#[derive(Debug)]
pub struct Event {
  name: String,
  data: HashMap<String, String>,
}
impl Event {
  pub fn new() -> Self {
    Event {
      name: String::from(""),
      data: HashMap::new(),
    }
  }
}
