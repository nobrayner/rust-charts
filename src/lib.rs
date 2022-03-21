mod action;
mod algorithm;
mod event;
mod machine_state;
mod machine;
mod schematic;
mod types;

pub mod prelude {
    pub use std::collections::HashMap;
    pub use std::any::TypeId;

    pub use crate::types::*;
    pub use crate::schematic::*;
    pub use crate::machine::{Machine, Parts};
    pub use crate::machine_state::MachineState;
}
