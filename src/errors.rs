use crate::util;

pub enum ErrorKind {
  // Lexer
  InvalidToken,
  InvalidNumberLiteral,
  UnfinishedString,

}

pub struct CodeDetails {
  line: String,
  position: util::Position
}

pub fn throw_error<T>(kind: ErrorKind, details: CodeDetails) -> Option<T> {
  

  None
}
