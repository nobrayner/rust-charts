use phf::OrderedMap;

use super::StateNode;

pub struct CompoundStateNode {
  pub id: &'static str,
  pub key: &'static str,
  pub on: OrderedMap<&'static str, &'static str>,
  pub initial: &'static str,
  pub states: OrderedMap<&'static str, &'static str>,
}
impl StateNode for CompoundStateNode {
  fn id(&self) -> String {
    String::from(self.id)
  }

  fn key(&self) -> String {
    String::from(self.key)
  }
}
