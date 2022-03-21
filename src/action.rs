use crate::event::EventData;

type ActionError = ();

type PlainAction<S, E, C> = fn(C, EventData<S, E>) -> Result<(), ActionError>;

type AssignAction<S, E, C> = fn(C, EventData<S, E>) -> Result<C, ActionError>;

// type SendAction<S, E, C> = ???
// type RaiseAction<S, E, C> = ???
