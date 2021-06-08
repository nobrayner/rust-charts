mod lib;

pub fn main() {
  let test_machine = lib::MachineConfig {
    initial: "start",
    states: vec![
      (
        "start",
        lib::State {
          on: Some(vec![
            (
              "MIDDLE",
              lib::Transition::GuardedTransition(lib::GuardedTransition {
                target: "middle",
                guard: Box::new(|| true),
              }),
            ),
            ("END", lib::Transition::Transition("end")),
          ]),
          on_enter: Some(Box::new(|| println!("Entering start!"))),
          on_exit: Some(Box::new(|| println!("Exiting start..."))),
        },
      ),
      (
        "middle",
        lib::State {
          on: Some(vec![("END", lib::Transition::Transition("end"))]),
          on_enter: None,
          on_exit: None,
        },
      ),
      (
        "end",
        lib::State {
          on: None,
          on_enter: Some(Box::new(|| println!("Entering end!"))),
          on_exit: None,
        },
      ),
    ],
  };

  let mut machine = lib::Machine::interpret(test_machine);

  machine.send("MIDDLE");
  machine.send("END");
}
