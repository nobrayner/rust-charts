use std::collections::HashMap;

#[derive(Debug)]
pub struct Machine<'a> {
    current_state: &'a str,
    states: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

impl Machine<'_> {
    pub fn new<'a>(
        initial: &'a str,
        states: Vec<(&'a str, Vec<(&'a str, &'a str)>)>,
    ) -> Machine<'a> {
        let states: HashMap<&str, HashMap<&str, &str>> = states
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        Machine {
            current_state: initial,
            states: states,
        }
    }

    pub fn send(&mut self, event: &str) -> () {
        self.current_state = self
            .states
            .get(self.current_state)
            .unwrap()
            .get(event)
            .unwrap();
    }
}
