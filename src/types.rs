use std::{hash::Hash, cmp::{PartialEq, Eq}};

pub trait StateIdentifier: Hash + PartialEq + Eq {}
pub trait GuardIdentifier: Hash + PartialEq + Eq {}
pub trait ActionIdentifier: Hash + PartialEq + Eq {}
pub trait ActivityIdentifier: Hash + PartialEq + Eq {}
pub trait EventIdentifier: Hash + PartialEq + Eq {}

pub trait EventWithId<E> where E: EventIdentifier {
    fn id(&self) -> E;
}

