use crate::event;

pub type GuardFn = fn(&event::Event) -> bool;
