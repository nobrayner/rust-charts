use rust_charts::prelude::*;

////////////////////////////
// States
////////////////////////////

#[derive(Hash, PartialEq, Eq)]
pub enum FsmState {
  On,
  Off,
}
impl StateTrait for FsmState {
    type Parent = ();

    fn parent() -> Option<Self::Parent> {
        None
    }

    fn kind(&self) -> StateKind {
        match self {
            _ => StateKind::Atomic,
        }
    }
}

////////////////////////////
// Events
////////////////////////////

pub enum FsmEvent {
    Toggle(Toggle),
}

pub struct Toggle;
impl EventPayload for Toggle {
    type Wrapper = FsmEvent;
}
impl From<Toggle> for FsmEvent {
    fn from(event: Toggle) -> Self {
        Self::Toggle(event)
    }
}

////////////////////////////
// Schematic
////////////////////////////

use lazy_static::lazy_static;
use std::marker::PhantomData;

lazy_static! {
    pub static ref FSM_SCHEMATIC: Schematic<FsmState, FsmEvent> = {
        let mut schematic = Schematic {
            initial_state: Some(FsmState::Off),
            states: HashMap::with_capacity(7),
        };

        schematic.states.insert(FsmState::On, State {
            _e: PhantomData,
            initial_state: None,
            document_order: 0,
            children: Vec::with_capacity(0),
            on: {
                let mut on = HashMap::with_capacity(1);

                on.insert(TypeId::of::<Toggle>(), Transition {
                    target: FsmState::Off,
                });

                on
            },
        });

        schematic.states.insert(FsmState::Off, State {
            _e: PhantomData,
            initial_state: None,
            document_order: 1,
            children: Vec::with_capacity(0),
            on: {
                let mut on = HashMap::with_capacity(1);

                on.insert(TypeId::of::<Toggle>(), Transition {
                    target: FsmState::On,
                });

                on
            },
        });

        schematic
    };
}
