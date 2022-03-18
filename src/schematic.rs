use crate::types::{StateTrait, EventPayload};
use std::{
    collections::HashMap,
    any::TypeId,
    marker::PhantomData,
};

pub enum StateKind {
    Atomic,
    Compound,
    Final,
    Parallel,
    History(HistoryKind),
}

pub enum HistoryKind {
    Shallow,
    Deep,
}

pub struct Schematic<S, E> where S: StateTrait {
    pub initial_state: Option<S>,
    pub states: HashMap<S, State<S, E>>,
}

pub struct State<S: StateTrait, E> {
    pub _e: PhantomData<E>,
    pub document_order: usize,
    // path: String,
    // name: String,
    pub initial_state: Option<S>,
    pub children: Vec<S>,
    // entry: ?{},
    // exit: ?{},
    // activities: ?{},
    // from: ?{},
    // transitions: ?{},
    pub on: HashMap<TypeId, Transition<S>>,
    // catch: ?{},
    // after: ?{},
    // now: ?{},
    // settled: ?{},
}

pub struct Transition<S: StateTrait> {
    pub target: S,
    // pub guard: Option<G>,
    // pub actions: Vec<A>,
}
