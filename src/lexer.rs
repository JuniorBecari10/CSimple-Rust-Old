use crate::{util, token};

pub struct Lexer {
  file_data: util::FileData,

  start: usize,
  cursor: usize,

  start_pos: util::Position,
  cursor_pos: util::Position
}

impl Lexer {
  pub fn new(file_data: util::FileData) -> Self {
    Self {
      file_data,
      
      start: 0,
      cursor: 0,

      start_pos: util::Position { line: 0, col: 0 },
      cursor_pos: util::Position { line: 0, col: 0 },
    }
  }

  pub fn lex(&mut self) -> Option<Vec<token::Token>> {
    let mut tokens: Vec<token::Token> = Vec::new();
    let mut had_error = false;



    Some(tokens)
  }

  // Auxiliary

  fn peek(&self, offset: usize) -> char {

  }

  fn advance(&mut self) -> char {

  }

  fn is_at_end(&self, offset: usize) -> bool {
    self.cursor + offset >= self.
  }
}

fn is_identifier(is_start: bool, c: char) -> bool {
  match is_start {
    true => c.is_alphabetic() || c == '_',
    false => c.is_alphabetic() || c.is_digit(10) || c == '_'
  }
}

fn is_number(is_start: bool, c: char) -> bool {
  match is_start {
    true => c.is_digit(10),
    false => c.is_digit(10) || c == '.'
  }
}
