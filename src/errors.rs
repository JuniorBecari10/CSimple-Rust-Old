use crate::util;

pub enum ErrorKind {
  // Lexer
  InvalidToken,
  InvalidNumberLiteral,
  UnfinishedString,

}

pub struct CodeDetails {
  pub line: String,
  pub position: util::Position
}

pub fn throw_error<T>(kind: ErrorKind, details: CodeDetails) -> Option<T> {
  let message = get_message(&kind, &details);

  None
}

fn get_message(kind: &ErrorKind, details: &CodeDetails) -> &'static str {
  match kind {
    ErrorKind::InvalidToken => "Invalid token",
    ErrorKind::InvalidNumberLiteral => "",
    ErrorKind::UnfinishedString => ""
  }
}
