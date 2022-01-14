use rust_charts::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightState {
    Green,
    Yellow,
    Red,
    RedWalk,
    RedWait,
    RedStop,
    RedTimeout,
}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightEvent {
    Timer,
    Countdown,
    Timeout,
}

// #[derive(Hash, PartialEq, Eq)]
// pub enum SimpleLightAction {}

// #[derive(Hash, PartialEq, Eq)]
// pub enum SimpleLightCondition {}

// #[derive(Hash, PartialEq, Eq)]
// pub enum SimpleLightActivity {}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SIMPLE_LIGHT_SCHEMATIC: Schematic<SimpleLightState, SimpleLightEvent> = {
        let mut builder = SchematicBuilder::new();

        builder.atomic_state(SimpleLightState::Green, |s| {
            s.name(String::from("Green State"));
            s.on(SimpleLightEvent::Timer, SimpleLightState::Yellow);
        });
        builder.atomic_state(SimpleLightState::Yellow, |s| {
            s.name(String::from("Yellow State"));
            s.on(SimpleLightEvent::Timer, SimpleLightState::Red);
        });

        builder.build()
    };
}
