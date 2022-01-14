use std::{hash::Hash, cmp::{Eq, PartialEq}};

pub trait EventIdentifier: Hash + PartialEq + Eq {}

pub enum EventKind {
    Platform,
    Internal,
    External,
}

pub struct Event<E: EventIdentifier> {
    data: E,
    origin: String,
    kind: EventKind,
    // Maybe other SCXML stuff
}
