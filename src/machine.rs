use crate::{
    schematic::Schematic,
    types::{StateTrait, EventPayload},
    machine_state::MachineState,
    algorithm
};
// use std::collections::HashMap;

// pub struct DummyAction<Ev, E, C> where Ev: EventWithId<E>, E: EventIdentifier {
//     _ev: Ev,
//     _e: E,
//     _c: C,
// }
// ActionId, ActivityId, GuardId, Context, Event, EventId
// pub struct Parts<Ev, E, C, Act, Atv, G>
// where
//     Act: ActionIdentifier, Atv: ActivityIdentifier, G: GuardIdentifier, Ev: EventWithId<E>, E: EventIdentifier
// {
//     /// FIXME: Have an actual error type here, instead of "empty"
//     /// Also, references are going to be an absolute bitch, here, most likely, as we need
//     /// functions/closures that take a context reference and event reference and do something with
//     /// it...
//     pub actions: HashMap<Act, DummyAction<Ev, E, C>>,
//     /// FIXME: Get a proper activity type
//     pub activities: HashMap<Atv, ()>,
//     /// FIXME: Get a proper activity type
//     pub guards: HashMap<G, ()>,
// }
pub struct Parts;

pub struct Machine<'a, S, E, C = ()>
where
    S: StateTrait, C: Default
{
    schematic: &'a Schematic<S, E>,
    context: C,
    // parts: &'a Parts<Ev, E, C, Act, Atv, G>,
}
impl <'a, S, E, C> Machine<'a, S, E, C>
where
    S: StateTrait, C: Default
{
    pub fn from(schematic: &'a Schematic<S, E> /*, parts: &'a Parts<Ev, E, C, Act, Atv, G>*/) -> Self {
        Self {
            schematic,
            context: Default::default(),
            // parts,
        }
    }

    pub fn transition<Ev>(&self, state: MachineState<S>, event: Ev) -> MachineState<S>
    where
        Ev: EventPayload<Wrapper = E> + Into<E> + 'static,
    {
        algorithm::event_loop_step(&self.schematic, state, event.into())
    }
}

