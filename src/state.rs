use crate::machine::Machine;

pub struct State {
    configuration: Vec<String>,
}
impl State {
    pub fn is_in(&self, state_id: &str) -> bool {
        self.configuration.iter().any(|c| c == state_id)
    }
    pub fn matches(&self, configuration: Vec<&str>) -> bool {
        self.configuration == configuration
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn is_in_helper() {
        let state = State { configuration: vec![String::from("a"), String::from("b")] };

        assert_eq!(true, state.is_in("b"));
    }

    #[test]
    fn matches_helper() {
        let state = State { configuration: vec![String::from("a"), String::from("b")] };

        assert_eq!(true, state.matches(vec!["a", "b"]))
    }
}

