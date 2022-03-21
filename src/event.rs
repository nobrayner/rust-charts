use crate::types::StateTrait;

// pub enum EventKind {
//     Platform,
//     Internal,
//     External,
// }

pub enum BuiltinEvent<S: StateTrait> {
    StateDone(S),
    // InvokeDone(I),
    CommunicationError,
    // These next two are... Not needed?
    // Though, the "ExecutionError" would make sense as the error type
    // coming from an action... Hmm...
    ExecutionError,
    PlatformError,
}

pub enum Event<S: StateTrait, E> {
    Builtin(BuiltinEvent<S>),
    Event(E),
}

pub struct EventData<S: StateTrait, E> {
    pub payload: Event<S, E>,
    pub origin: String,
    // kind: EventKind,
    // Maybe other SCXML stuff
}

