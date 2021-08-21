use std::fmt;
// use std::{collections::HashMap, fmt};

// use crate::{action::Action, transition::Transition};

mod atomic;
mod compound;

pub use atomic::*;
pub use compound::*;

pub trait StateNode {
  fn id(&self) -> String;
  fn key(&self) -> String;
}

pub enum StateNodeKind {
  Atomic(AtomicStateNode),
  Compound(CompoundStateNode),
}
impl fmt::Debug for StateNodeKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Atomic(s) => write!(f, "<StateNode \"{}\">", &s.id),
      Self::Compound(s) => write!(f, "<StateNode \"{}\">", &s.id),
    }
  }
}
