use crate::util;

pub enum ErrorKind {
  // Lexer
  InvalidToken,
  InvalidNumberLiteral,
  UnfinishedString,

}

pub struct CodeDetails {
  pub line: Vec<char>,
  pub pos: util::Position
}

pub fn throw_error<T>(kind: &ErrorKind, details: &CodeDetails, data: &util::FileData) -> Option<T> {
  let message = get_message(&kind, &details);

  println!("(X) Error in {} | {}:{}\n", data.file_name, details.pos.line + 1, details.pos.col + 1);
  println!("{}\n", message);
  println!(" {} | {}", details.pos.line + 1, details.line.iter().cloned().collect::<String>());

  None
}

fn get_message(kind: &ErrorKind, details: &CodeDetails) -> String {
  match kind {
    ErrorKind::InvalidToken => format!("Invalid token: '{}'\nConsider removing it.", details.line[details.pos.col]),
    ErrorKind::InvalidNumberLiteral => "".into(),
    ErrorKind::UnfinishedString => "".into()
  }
}
