use phf::OrderedMap;

use super::StateNode;

pub struct AtomicStateNode {
  pub id: &'static str,
  pub key: &'static str,
  pub on: OrderedMap<&'static str, &'static str>,
}
impl StateNode for AtomicStateNode {
  fn id(&self) -> String {
    String::from(self.id)
  }

  fn key(&self) -> String {
    String::from(self.key)
  }
}
