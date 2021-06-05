mod lib;

pub fn main() {
  let mut machine = lib::Machine::new(
    "start",
    vec![
      (
        "start",
        lib::State {
          on: Some(vec![("MIDDLE", "middle"), ("END", "end")]),
          on_enter: Some(Box::new(|| println!("Entering start!"))),
          on_exit: Some(Box::new(|| println!("Exiting start..."))),
        },
      ),
      (
        "middle",
        lib::State {
          on: Some(vec![("END", "end")]),
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
  );

  machine.send("WRONG");
  machine.send("END");
}
