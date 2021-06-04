mod lib;

pub fn main() {
  let mut machine = lib::Machine::new(
    "start",
    vec![
      ("start", vec![("MIDDLE", "middle"), ("END", "end")]),
      ("middle", vec![("END", "end")]),
      ("end", vec![]),
    ],
  );

  machine.send("MIDDLE");
  machine.send("END");
}
