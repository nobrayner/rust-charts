use crate::types::StateIdentifier;

pub struct MachineState<S> where S: StateIdentifier {
    configuration: Vec<S>,
}
impl <S> MachineState<S> where S: StateIdentifier {
    pub fn from(configuration: Vec<S>) -> Self {
        Self {
            configuration,
        }
    }
    pub fn is_in(&self, state_id: S) -> bool {
        self.configuration.iter().any(|c| *c == state_id)
    }
    pub fn matches(&self, configuration: Vec<S>) -> bool {
        self.configuration == configuration
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::hash::Hash;
    use std::cmp::{PartialEq, Eq};

    #[derive(Hash, PartialEq, Eq)]
    enum ConfigState {
        A,
        B,
    }
    impl StateIdentifier for ConfigState {}

    #[test]
    fn is_in_helper() {
        let state = MachineState { configuration: vec![ConfigState::A, ConfigState::B] };

        assert_eq!(true, state.is_in(ConfigState::B));
    }

    #[test]
    fn matches_helper() {
        let state = MachineState { configuration: vec![ConfigState::A, ConfigState::B] };

        assert_eq!(true, state.matches(vec![ConfigState::A, ConfigState::B]))
    }
}

