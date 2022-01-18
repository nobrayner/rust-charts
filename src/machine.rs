use crate::{
    schematic::Schematic,
    types::{StateIdentifier, EventIdentifier, EventWithId, ActionIdentifier, ActivityIdentifier, GuardIdentifier},
    machine_state::MachineState,
};
use std::collections::HashMap;

pub struct DummyAction<Ev, E, C> where Ev: EventWithId<E>, E: EventIdentifier {
    _ev: Ev,
    _e: E,
    _c: C,
}
// ActionId, ActivityId, GuardId, Context, Event, EventId
pub struct Parts<Ev, E, C, Act, Atv, G>
where
    Act: ActionIdentifier, Atv: ActivityIdentifier, G: GuardIdentifier, Ev: EventWithId<E>, E: EventIdentifier
{
    /// FIXME: Have an actual error type here, instead of "empty"
    /// Also, references are going to be an absolute bitch, here, most likely, as we need
    /// functions/closures that take a context reference and event reference and do something with
    /// it...
    pub actions: HashMap<Act, DummyAction<Ev, E, C>>,
    /// FIXME: Get a proper activity type
    pub activities: HashMap<Atv, ()>,
    /// FIXME: Get a proper activity type
    pub guards: HashMap<G, ()>,
}

pub struct Machine<'a, S, Ev, E, C, Act, Atv, G>
where
    S: StateIdentifier, Ev: EventWithId<E>, E: EventIdentifier, Act: ActionIdentifier, Atv: ActivityIdentifier, G: GuardIdentifier
{
    schematic: &'a Schematic<S, Ev, E>,
    parts: &'a Parts<Ev, E, C, Act, Atv, G>,
}
impl <'a, S, Ev, E, C, Act, Atv, G> Machine<'a, S, Ev, E, C, Act, Atv, G>
where
    S: StateIdentifier, Ev: EventWithId<E>, E: EventIdentifier, Act: ActionIdentifier, Atv: ActivityIdentifier, G: GuardIdentifier
{
    pub fn from(schematic: &'a Schematic<S, Ev, E>, parts: &'a Parts<Ev, E, C, Act, Atv, G>) -> Self {
        Self {
            schematic,
            parts,
        }
    }

    pub fn transition(&self, state: MachineState<S>, event: Ev) -> MachineState<S> {
        state
    }
}

