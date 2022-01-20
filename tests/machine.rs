use rust_charts::*;

mod simple_light_machine;
use simple_light_machine::*;

#[test]
pub fn atomic_transitions() {
    let machine = Machine::from(&SIMPLE_LIGHT_SCHEMATIC, &SIMPLE_LIGHT_PARTS);

    let start_state = MachineState::from_configuration(vec![SimpleLightState::Green]);
    let next_state = machine.transition(start_state, SimpleLightEvent::Timer);

    assert!(next_state.is_in(SimpleLightState::Yellow));
}
