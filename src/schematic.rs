use crate::event::EventIdentifier;
use std::{collections::HashMap, hash::Hash, cmp::{PartialEq, Eq}};

pub trait StateIdentifier: Hash + PartialEq + Eq {}

pub struct Schematic<S: StateIdentifier, E: EventIdentifier> {
    states: HashMap<S, State<S, E>>,
}
impl <S: StateIdentifier, E: EventIdentifier> Schematic<S, E> {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn build(self) -> Schematic<S, E> {
        Schematic {
            states: self.states,
        }
    }

    pub fn atomic_state<B>(&mut self, identifier: S, builder: B) -> &mut Self where B: Fn(&mut State<S, E>) -> () {
        let mut state = State {
            parent: None,
            document_order: self.states.len() as u16,
            kind: StateKind::Atomic,
            // initial_state: None,
            // children: vec![],
            // history_kind: None,
            on: HashMap::new(),
        };
        builder(&mut state);

        self.states.insert(identifier, state);

        self
    }
}

pub enum StateKind {
    Atomic,
    Compound,
    Final,
    Parallel,
    History,
}

pub enum HistoryKind {
    Shallow,
    Deep,
}

type DocumentOrder = u16;

pub struct State<S: StateIdentifier, E: EventIdentifier> {
    parent: Option<S>,
    document_order: DocumentOrder,
    kind: StateKind,
    // path: String,
    // name: String,
    // initial_state: Option<S>,
    // children: Vec<S>,
    // history_kind: Option<HistoryKind>,
    // entry: ?{},
    // exit: ?{},
    // activities: ?{},
    // from: ?{},
    // transitions: ?{},
    on: HashMap<E, S>,
    // catch: ?{},
    // after: ?{},
    // now: ?{},
    // settled: ?{},
}
impl <S: StateIdentifier, E: EventIdentifier> State<S, E> {
    pub fn on(&mut self, event: E, target: S) {
        self.on.insert(event, target);
    }
}
