use crate::types::EventPayload;

pub enum EventKind {
    Platform,
    Internal,
    External,
}

pub struct Event<E: EventPayload> {
    payload: E::Wrapper,
    origin: String,
    kind: EventKind,
    // Maybe other SCXML stuff
}

