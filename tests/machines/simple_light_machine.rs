use rust_charts::prelude::*;

////////////////////////////
// States
////////////////////////////

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightState {
  Green,
  Yellow,
  Red(Option<SimpleLightRedState>),
}
impl StateTrait for SimpleLightState {
    type Parent = ();

    fn parent() -> Option<Self::Parent> {
        None
    }

    fn kind(&self) -> StateKind {
        match self {
            Self::Red(_) => StateKind::Compound,
            _ => StateKind::Atomic,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum SimpleLightRedState {
    Walk,
    Wait,
    Stop,
    Timeout,
}
impl StateTrait for SimpleLightRedState {
    type Parent = SimpleLightState;

    fn parent() -> Option<Self::Parent> {
        Some(SimpleLightState::Red(None))
    }

    fn kind(&self) -> StateKind {
        match self {
            Self::Timeout => StateKind::Final,
            _ => StateKind::Atomic,
        }
    }
}

////////////////////////////
// Events
////////////////////////////

pub enum SimpleLightEvent {
    Timer(Timer),
    Countdown(Countdown),
    Timeout(Timeout),
}

pub struct Timer;
impl EventPayload for Timer {
    type Wrapper = SimpleLightEvent;
}
impl From<Timer> for SimpleLightEvent {
    fn from(event: Timer) -> Self {
        Self::Timer(event)
    }
}

pub struct Countdown;
impl EventPayload for Countdown {
    type Wrapper = SimpleLightEvent;
}
impl From<Countdown> for SimpleLightEvent {
    fn from(event: Countdown) -> Self {
        Self::Countdown(event)
    }
}

pub struct Timeout;
impl EventPayload for Timeout {
    type Wrapper = SimpleLightEvent;
}
impl From<Timeout> for SimpleLightEvent {
    fn from(event: Timeout) -> Self {
        Self::Timeout(event)
    }
}

////////////////////////////
// Schematic
////////////////////////////

use lazy_static::lazy_static;
use std::marker::PhantomData;

lazy_static! {
    pub static ref SIMPLE_LIGHT_SCHEMATIC: Schematic<SimpleLightState, SimpleLightEvent> = {
        let mut schematic = Schematic {
            initial_state: Some(SimpleLightState::Green),
            states: HashMap::with_capacity(7),
        };

        schematic.states.insert(SimpleLightState::Green, State {
            _e: PhantomData,
            initial_state: None,
            document_order: 0,
            children: Vec::with_capacity(0),
            on: {
                let mut on = HashMap::with_capacity(1);

                on.insert(TypeId::of::<Timer>(), Transition {
                    target: SimpleLightState::Yellow,
                });

                on
            },
        });

        schematic.states.insert(SimpleLightState::Yellow, State {
            _e: PhantomData,
            initial_state: None,
            document_order: 0,
            children: Vec::with_capacity(0),
            on: {
                let mut on = HashMap::with_capacity(1);

                on.insert(TypeId::of::<Timer>(), Transition {
                    target: SimpleLightState::Red(None),
                });

                on
            },
        });

        schematic.states.insert(SimpleLightState::Red(None), State {
            _e: PhantomData,
            initial_state: Some(
                SimpleLightState::Red(Some(SimpleLightRedState::Walk))
            ),
            document_order: 0,
            children: vec![
                SimpleLightState::Red(Some(SimpleLightRedState::Walk)),
                SimpleLightState::Red(Some(SimpleLightRedState::Wait)),
                SimpleLightState::Red(Some(SimpleLightRedState::Stop)),
                SimpleLightState::Red(Some(SimpleLightRedState::Timeout)),
            ],
            on: HashMap::with_capacity(0),
        });

        schematic.states.insert(
            SimpleLightState::Red(Some(SimpleLightRedState::Walk)),
            State {
                _e: PhantomData,
                initial_state: None,
                document_order: 0,
                children: Vec::with_capacity(0),
                on: {
                    let mut on = HashMap::with_capacity(1);

                    on.insert(TypeId::of::<Countdown>(), Transition {
                        target: SimpleLightState::Red(
                            Some(SimpleLightRedState::Wait)
                        ),
                    });

                    on
                },
            }
        );

        schematic.states.insert(
            SimpleLightState::Red(Some(SimpleLightRedState::Wait)),
            State {
                _e: PhantomData,
                initial_state: None,
                document_order: 0,
                children: Vec::with_capacity(0),
                on: {
                    let mut on = HashMap::with_capacity(1);

                    on.insert(TypeId::of::<Countdown>(), Transition {
                        target: SimpleLightState::Red(
                            Some(SimpleLightRedState::Stop)
                        ),
                    });

                    on
                },
            }
        );

        schematic.states.insert(
            SimpleLightState::Red(Some(SimpleLightRedState::Stop)),
            State {
                _e: PhantomData,
                initial_state: None,
                document_order: 0,
                children: Vec::with_capacity(0),
                on: {
                    let mut on = HashMap::with_capacity(1);

                    on.insert(TypeId::of::<Timeout>(), Transition {
                        target: SimpleLightState::Red(
                            Some(SimpleLightRedState::Timeout)
                        ),
                    });

                    on
                },
            }
        );

        schematic.states.insert(
            SimpleLightState::Red(Some(SimpleLightRedState::Timeout)),
            State {
                _e: PhantomData,
                initial_state: None,
                document_order: 0,
                children: Vec::with_capacity(0),
                on: HashMap::with_capacity(0),
            }
        );

        schematic
    };
}
