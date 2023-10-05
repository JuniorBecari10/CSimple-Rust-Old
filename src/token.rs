use crate::util;

pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub literal: Literal,
  pub pos: util::Position
}

pub enum Literal {
  Num(f64),
  Str(String),
  Bool(bool)
}

pub enum TokenKind {
  Identifier,
  Number,
  String,

  Plus,
  Minus,
  Times,
  Divide,

  Modulo,

  LParen,
  RParen,

  Assign,
  Bang,

  PlusAssign,
  MinusAssign,
  TimesAssign,
  DivideAssign,

  ModuloAssign,

  Equal,
  NotEqual,

  Less,
  Greater,

  LessEqual,
  GreaterEqual,

  PrintlnKw,
  PrintKw,
  InputKw,
  GotoKw,
  RunKw,
  RetKw,
  ExecKw,
  ExitKw,
  IfKw,
  ElseKw
}
