use crate::{util, token::{self, TokenKind}};

macro_rules! ope_or_assign {
  ($self: expr, $a: expr, $b: expr) => {
    {
      if $self.peek(0) == '=' { Some($self.new_token_char($a)) }
      else { Some($self.new_token_char($b)) }
    }
  }
}

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
      tokens.push(self.new_token_statend(self.cursor_pos.clone()));
    }

    Some(tokens)
  }

  fn token(&mut self) -> Option<token::Token> {
    self.skip_whitespace();

    self.start = self.cursor;
    self.start_pos = self.cursor_pos.clone();

    let c = self.advance();

    match c {
      '+' => ope_or_assign!(self, token::TokenKind::PlusAssign, token::TokenKind::Plus),
      '-' => ope_or_assign!(self, token::TokenKind::MinusAssign, token::TokenKind::Minus),
      '*' => ope_or_assign!(self, token::TokenKind::TimesAssign, token::TokenKind::Times),
      '/' => ope_or_assign!(self, token::TokenKind::DivideAssign, token::TokenKind::Divide),
      '%' => ope_or_assign!(self, token::TokenKind::ModuloAssign, token::TokenKind::Modulo),

      '(' => Some(self.new_token_char(token::TokenKind::LParen)),
      ')' => Some(self.new_token_char(token::TokenKind::RParen)),

      '&' => ope_or_assign!(self, token::TokenKind::AndAssign, token::TokenKind::And),
      '|' => ope_or_assign!(self, token::TokenKind::OrAssign, token::TokenKind::Or),

      '=' => ope_or_assign!(self, token::TokenKind::Assign, token::TokenKind::Equal),
      '!' => ope_or_assign!(self, token::TokenKind::Bang, token::TokenKind::NotEqual),

      '>' => ope_or_assign!(self, token::TokenKind::GreaterEqual, token::TokenKind::Greater),
      '<' => ope_or_assign!(self, token::TokenKind::LessEqual, token::TokenKind::Less),

      '#' => {
        let mut old = self.cursor_pos.clone();
        old.col -= 1;

        while !self.is_at_end(0) && self.peek(0) != '\n' {
          self.advance();
        }

        self.advance();

        self.cursor_pos.col = 1;
        self.cursor_pos.line += 1;

        Some(self.new_token_statend(old))
      }

      '\n' => {
        let old = self.cursor_pos.clone();

        self.cursor_pos.col = 1;
        self.cursor_pos.line += 1;

        Some(self.new_token_statend(old))
      }

      _ => {
        if is_identifier(true, c) {
          while !self.is_at_end(0) && is_identifier(false, self.peek(0)) {
            self.advance();
          }

          let lexeme: String = self.input.clone().drain(self.start..self.cursor).collect();
          let kind = match token::KEYWORDS.get(lexeme.as_str()) {
            Some(k) => *k,
            None => token::TokenKind::Identifier
          };

          match kind {
            token::TokenKind::NilKw => Some(self.new_token_literal(kind, token::Literal::Nil)),
            token::TokenKind::TrueKw | token::TokenKind::FalseKw => Some(self.new_token_literal(kind, token::Literal::Bool(kind == TokenKind::TrueKw))),

            _ => Some(self.new_token_str(kind))
          }
        }

        else if is_number(true, c) {
          while !self.is_at_end(0) && is_number(false, self.peek(0)) {
            self.advance();
          }

          let num = match self.input.clone().drain(self.start..self.cursor).collect::<String>().parse::<f64>() {
              Ok(n) => n,
              Err(_) => {
                // todo! throw error
                return None;
              }
          };

          Some(self.new_token_literal(token::TokenKind::Number, token::Literal::Num(num)))
        }

        else if c == '"' {
          while !self.is_at_end(0) && self.peek(0) != '"' {
            self.advance();
          }

          if !self.match_adv('"') {
            // todo! throw error
          }

          let new_pos = self.start_pos.clone();
          new_pos.col += 1;

          let lexeme: String = self.input.clone().drain(self.start + 1..self.cursor - 1).collect();

          Some(token::Token {
            kind: token::TokenKind::String,
            lexeme: lexeme.clone(),
            literal: token::Literal::Str(lexeme),
            pos: self.start_pos.clone()
          })
        }

        else {
          // todo! throw error
        }
      }
    }
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

  fn match_adv(&mut self, c: char) -> bool {
    let ch = self.peek(0);
    self.advance();

    ch == c
  }

  fn is_at_end(&self, offset: usize) -> bool {
    self.cursor + offset >= self.input.len()
  }

  fn skip_whitespace(&mut self) {
    while self.peek(0).is_whitespace() && self.peek(0) != '\n' {
      self.advance();
    }
  }

  // New Token

  fn new_token_char(&self, kind: token::TokenKind) -> token::Token {
    token::Token {
      kind,
      lexeme: self.peek(0).to_string(),
      literal: token::Literal::Str(self.peek(0).to_string()),
      pos: self.start_pos.clone()
    }
  }

  fn new_token_str(&self, kind: token::TokenKind) -> token::Token {
    let lexeme: String = self.input.clone().drain(self.start..self.cursor).collect();

    token::Token {
      kind,
      lexeme: lexeme.clone(),
      literal: token::Literal::Str(lexeme),
      pos: self.start_pos.clone()
    }
  }

  fn new_token_literal(&self, kind: token::TokenKind, literal: token::Literal) -> token::Token {
    let lexeme: String = self.input.clone().drain(self.start..self.cursor).collect();

    token::Token {
      kind,
      lexeme: lexeme.clone(),
      literal,
      pos: self.start_pos.clone()
    }
  }

  fn new_token_statend(&self, pos: util::Position) -> token::Token {
    token::Token {
      kind: token::TokenKind::StatEnd,
      lexeme: "".into(),
      literal: token::Literal::Str("".into()),
      pos
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
