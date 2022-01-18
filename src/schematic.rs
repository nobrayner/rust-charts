use crate::types::{StateIdentifier, EventIdentifier, EventWithId};
use std::{
    collections::HashMap,
    marker::PhantomData,
};

pub struct Schematic<S, Ev, E> where S: StateIdentifier, Ev: EventWithId<E>, E: EventIdentifier {
    states: HashMap<S, State<S, E>>,
    __event: PhantomData<Ev>,
}
impl <S, Ev, E> Schematic<S, Ev, E> where S: StateIdentifier, Ev: EventWithId<E>, E: EventIdentifier {
    pub fn new() -> Self {
        Self {
            __event: PhantomData,
            states: HashMap::new(),
        }
    }

    pub fn build(self) -> Schematic<S, Ev, E> {
        Schematic {
            __event: PhantomData,
            states: self.states,
        }
    }

    pub fn atomic_state<B>(&mut self, identifier: S, builder: B) -> &mut Self where B: Fn(&mut State<S, E>) -> () {
        let mut state = State {
            parent: None,
            document_order: self.states.len(),
            kind: StateKind::Atomic,
            name: self.states.len().to_string(),
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

pub struct State<S: StateIdentifier, E: EventIdentifier> {
    parent: Option<S>,
    document_order: usize,
    kind: StateKind,
    // path: String,
    name: String,
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
    pub fn name(&mut self, name: String) {
        self.name = name;
    }
}

