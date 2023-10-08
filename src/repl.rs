use crate::util;

pub fn repl() -> Option<()> {
  println!("CSimple REPL\n");
  let mut input;

  loop {
    input = util::input("> ");

    match input.as_str() {
      "" => continue,
      "exit" => return Some(()),

      _ => {}
    }

    let data = util::FileData {
      file_name: "REPL".to_string(),
      lines: vec![input.clone()]
    };

    util::build(&input, &data)?;
  }
}
