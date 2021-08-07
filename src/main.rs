use xstate_rust::{Machine, MachineConfig};

pub fn main() {
  let test_machine = Machine::new(MachineConfig {
    id: "test",
    states: vec![("on", "off"), ("off", "on")],
  });

  println!("{:?}", test_machine);
}
