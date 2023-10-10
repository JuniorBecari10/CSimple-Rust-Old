use std::io::{self, Write};

#[derive(Clone)]
pub struct FileData {
  pub file_name: String,
  pub lines: Vec<String>
}

#[derive(Clone, PartialEq)]
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

pub fn build(input: &str, file_data: &FileData) -> Option<()> {

  Some(())
}
