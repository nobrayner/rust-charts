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
impl StateIdentifier for SimpleLightState {}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightEventId {
    Timer,
    Countdown,
    Timeout,
}
impl EventIdentifier for SimpleLightEventId {}

pub enum SimpleLightEvent {
    Timer,
    Countdown,
    Timeout,
}
impl EventWithId<SimpleLightEventId> for SimpleLightEvent {
    fn id(&self) -> SimpleLightEventId {
        match self {
            Self::Timer => SimpleLightEventId::Timer,
            Self::Countdown => SimpleLightEventId::Countdown,
            Self::Timeout => SimpleLightEventId::Timeout,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightAction {}
impl ActionIdentifier for SimpleLightAction {}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightActivity {}
impl ActivityIdentifier for SimpleLightActivity {}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightGuard {}
impl GuardIdentifier for SimpleLightGuard {}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SIMPLE_LIGHT_SCHEMATIC: Schematic<SimpleLightState, SimpleLightEvent, SimpleLightEventId> = {
        let mut builder: Schematic<SimpleLightState, SimpleLightEvent, SimpleLightEventId> = Schematic::new();

        builder.atomic_state(SimpleLightState::Green, |s| {
            s.name(String::from("Green State"));
            s.on(SimpleLightEventId::Timer, SimpleLightState::Yellow);
        });
        builder.atomic_state(SimpleLightState::Yellow, |s| {
            s.name(String::from("Yellow State"));
            s.on(SimpleLightEventId::Timer, SimpleLightState::Red);
        });

        builder.build()
    };
}

lazy_static! {
    pub static ref SIMPLE_LIGHT_PARTS: Parts<SimpleLightEvent, SimpleLightEventId, (), SimpleLightAction, SimpleLightActivity, SimpleLightGuard> = {
        Parts {
            actions: HashMap::new(),
            activities: HashMap::new(),
            guards: HashMap::new(),
        }
    };
}
