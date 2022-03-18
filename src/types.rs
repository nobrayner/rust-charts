use crate::schematic::StateKind;
use std::{hash::Hash, cmp::{PartialEq, Eq}};

pub trait StateTrait: Hash + PartialEq + Eq {
    type Parent;

    fn parent() -> Option<Self::Parent>;
    fn kind(&self) -> StateKind;
}

pub trait EventPayload {
    type Wrapper;
}

// pub trait GuardIdentifier: Hash + PartialEq + Eq {}
// pub trait ActionIdentifier: Hash + PartialEq + Eq {}
// pub trait ActivityIdentifier: Hash + PartialEq + Eq {}
