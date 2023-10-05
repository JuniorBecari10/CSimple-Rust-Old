use std::io::{self, Write};

pub struct FileData {
  pub file_name: String,
  pub lines: Vec<String>
}

pub struct Position {
  pub line: usize,
  pub col: usize
}

pub fn input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut out = String::new();
  io::stdin().read_line(&mut out).unwrap();

  out
}

pub fn build(file_data: &FileData) -> Option<()> {

  Some(())
}
