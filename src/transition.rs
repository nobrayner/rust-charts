use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TransitionKind {
  External,
  Internal,
}

#[derive(PartialEq)]
pub struct Transition {
  targets: Vec<String>,
  actions: Vec<String>,
  guard: Option<String>,
  kind: TransitionKind,
  source: String,
}
impl Transition {
  pub fn new(
    targets: Vec<String>,
    actions: Vec<String>,
    guard: Option<String>,
    kind: TransitionKind,
    source: String,
  ) -> Self {
    Self {
      targets,
      actions,
      guard,
      kind,
      source,
    }
  }
  pub fn targets(&self) -> &[String] {
    &self.targets
  }
  pub fn actions(&self) -> &[String] {
    &self.actions
  }
  pub fn guard_id(&self) -> Option<&String> {
    self.guard.as_ref()
  }
  pub fn kind(&self) -> &TransitionKind {
    &self.kind
  }
  pub fn source(&self) -> &String {
    &self.source
  }
}
impl fmt::Debug for Transition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Transition")
      .field("targets", &self.targets)
      .field("actions", &self.actions)
      // This can't actually be displayed?
      .field(
        "guard",
        match &self.guard {
          Some(_) => &"fn (Event) -> bool",
          None => &"None",
        },
      )
      .field("kind", &self.kind)
      .field("source", &self.source)
      .finish()
  }
}
