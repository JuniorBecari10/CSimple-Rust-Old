use crate::{util, token};

pub struct Lexer {
  input: Vec<char>,
  file_data: util::FileData,

  start: usize,
  cursor: usize,

  start_pos: util::Position,
  cursor_pos: util::Position
}

impl Lexer {
  pub fn new(input: &str, file_data: util::FileData) -> Self {
    Self {
      input: input.chars().collect(),
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

    while !self.is_at_end(0) {
      let token = match self.token() {
        Some(t) => t,
        None => {
          had_error = true;
          continue;
        }
      };

      tokens.push(token);
    }

    if had_error {
      return None;
    }

    let mut pos = self.cursor_pos.clone();
    pos.col += 1;

    if tokens[tokens.len() - 1].kind != token::TokenKind::StatEnd {
      tokens.push(token::Token {
        kind: token::TokenKind::StatEnd,
        lexeme: "".into(),
        literal: token::Literal::Str("".into()),
        pos: self.cursor_pos.clone()
      });
    }

    Some(tokens)
  }

  fn token(&mut self) -> Option<token::Token> {
    
  }

  // Auxiliary

  fn peek(&self, offset: usize) -> char {
    if self.is_at_end(offset) {
      '\0'
    }
    else {
      self.input[self.cursor + offset]
    }
  }

  fn advance(&mut self) -> char {
    let c = self.peek(0);
    self.cursor += 1;

    if !self.is_at_end(0) && c != '\n' {
      self.cursor_pos.col += 1;
    }

    c
  }

  fn is_at_end(&self, offset: usize) -> bool {
    self.cursor + offset >= self.input.len()
  }

  fn skip_whitespace(&mut self) {
    while self.peek(0).is_whitespace() && self.peek(0) != '\n' {
      self.advance();
    }
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
