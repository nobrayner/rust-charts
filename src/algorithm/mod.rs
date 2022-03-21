use crate::{
    machine_state::MachineState,
    schematic::Schematic,
    types::StateTrait,
    event::EventData,
};

pub fn event_loop_step<S, E>(schematic: &Schematic<S, E>, state: MachineState<S>, event: EventData<S, E>) -> MachineState<S>
where
    S: StateTrait
{
    MachineState::new()
}
