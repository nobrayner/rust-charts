use crate::types::EventIdentifier;

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

