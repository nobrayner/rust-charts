use std::collections::HashMap;

// pub struct GuardedTransition<'a> {
//     target: &'a str,
//     guard: dyn Fn() -> bool
// }

// pub enum Transition<'a> {
//     State(&'a str),
// }

struct StateNode<'a> {
    on: Option<HashMap<&'a str, &'a str>>,
    on_enter: Option<Box<dyn Fn() -> ()>>,
    on_exit: Option<Box<dyn Fn() -> ()>>,
}
impl std::fmt::Debug for StateNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").field("on", &self.on).finish()
    }
}

pub struct State<'a> {
    pub on: Option<Vec<(&'a str, &'a str)>>,
    pub on_enter: Option<Box<dyn Fn() -> ()>>,
    pub on_exit: Option<Box<dyn Fn() -> ()>>,
}

#[derive(Debug)]
pub struct Machine<'a> {
    current_state: &'a str,
    states: HashMap<&'a str, StateNode<'a>>,
}

impl Machine<'_> {
    pub fn new<'a>(initial: &'a str, states: Vec<(&'a str, State<'a>)>) -> Machine<'a> {
        let states: HashMap<&str, StateNode> = states
            .into_iter()
            .map(|(state_name, state_config)| {
                let transition_config = state_config.on;

                (
                    state_name,
                    StateNode {
                        on: if transition_config.is_some() {
                            Some(transition_config.unwrap().into_iter().collect())
                        } else {
                            None
                        },
                        on_enter: state_config.on_enter,
                        on_exit: state_config.on_exit,
                    },
                )
            })
            .collect();

        let on_enter = states.get(initial).unwrap().on_enter.as_ref();

        if on_enter.is_some() {
            on_enter.unwrap()();
        }

        Machine {
            current_state: initial,
            states,
        }
    }

    pub fn send(&mut self, event: &str) -> () {
        let exiting_state = &self.states.get(&self.current_state).unwrap();
        let on_exit = exiting_state.on_exit.as_ref();

        let entering_state = exiting_state.on.as_ref().unwrap().get(event);

        if entering_state.is_some() {
            let entering_state = entering_state.unwrap();

            let on_enter = &self.states.get(entering_state).unwrap().on_enter.as_ref();
            if on_exit.is_some() {
                on_exit.unwrap()();
            }
            if on_enter.is_some() {
                on_enter.unwrap()();
            }
            self.current_state = entering_state;
        }
    }
}
