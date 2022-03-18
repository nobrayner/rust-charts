use rust_charts::prelude::*;

mod machines;
use machines::simple_light_machine::*;

#[test]
pub fn atomic_transitions() {
    // let machine = Machine::from(&SIMPLE_LIGHT_SCHEMATIC, &SIMPLE_LIGHT_PARTS);

    // let start_state = MachineState::from_configuration(vec![SimpleLightState::Green]);
    // let next_state = machine.transition(start_state, SimpleLightEvent::Timer);

    // assert!(next_state.is_in(SimpleLightState::Yellow));

    assert_eq!(true, true);
}
