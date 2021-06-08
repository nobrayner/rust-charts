use std::collections::HashMap;

pub struct GuardedTransition<'transition> {
  pub target: &'transition str,
  pub guard: Box<dyn Fn() -> bool>,
}
impl std::fmt::Debug for GuardedTransition<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Transition")
      .field("target", &self.target)
      .finish()
  }
}

#[derive(Debug)]
pub enum Transition<'transition> {
  Transition(&'transition str),
  GuardedTransition(GuardedTransition<'transition>),
}
impl<'a> Transition<'a> {
  pub fn target(&self) -> &'a str {
    match &self {
      Transition::Transition(str) => &str,
      Transition::GuardedTransition(transition) => &transition.target,
    }
  }
}

pub struct State<'state> {
  pub on: Option<Vec<(&'state str, Transition<'state>)>>,
  pub on_enter: Option<Box<dyn Fn() -> ()>>,
  pub on_exit: Option<Box<dyn Fn() -> ()>>,
}

struct StateNode<'node> {
  on: Option<HashMap<&'node str, Transition<'node>>>,
  on_enter: Option<Box<dyn Fn() -> ()>>,
  on_exit: Option<Box<dyn Fn() -> ()>>,
}
impl std::fmt::Debug for StateNode<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("State").field("on", &self.on).finish()
  }
}

pub struct MachineConfig<'config> {
  pub initial: &'config str,
  pub states: Vec<(&'config str, State<'config>)>,
}

pub struct Machine<'machine> {
  current_state: &'machine str,
  states: HashMap<&'machine str, StateNode<'machine>>,
}
impl<'machine> Machine<'machine> {
  pub fn interpret(config: MachineConfig<'machine>) -> Self {
    let states: HashMap<_, _> = config
      .states
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

    let on_enter = states.get(config.initial).unwrap().on_enter.as_ref();

    if on_enter.is_some() {
      on_enter.unwrap()();
    }

    Self {
      current_state: config.initial,
      states,
    }
  }

  pub fn send(&mut self, event: &str) -> Option<()> {
    let exiting_state = self.states.get(self.current_state).unwrap();
    let on_exit = exiting_state.on_exit.as_ref();

    let entering_state = exiting_state.on.as_ref()?.get(event)?.target();

    let on_enter = match self.states.get(entering_state) {
      Some(state) => state.on_enter.as_ref(),
      None => panic!(
        "Provided invalid state \"{}\". Possible options were: {:?}",
        entering_state,
        self.states.keys()
      ),
    };

    // Run exit/enter
    if on_exit.is_some() {
      on_exit.unwrap()();
    }
    if on_enter.is_some() {
      on_enter.unwrap()();
    }

    // Update state
    self.current_state = entering_state;

    Some(())
  }
}
