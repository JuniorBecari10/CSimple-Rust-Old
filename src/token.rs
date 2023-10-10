extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::util;

lazy_static! {
  pub static ref KEYWORDS: HashMap<&'static str, TokenKind> = HashMap::from([
    ("println", TokenKind::PrintlnKw),
    ("print", TokenKind::PrintKw),
    ("input", TokenKind::InputKw),
    ("goto", TokenKind::GotoKw),
    ("run", TokenKind::RunKw),
    ("ret", TokenKind::RetKw),
    ("exec", TokenKind::ExecKw),
    ("exit", TokenKind::ExitKw),
    ("if", TokenKind::IfKw),
    ("else", TokenKind::ElseKw),
    ("nil", TokenKind::NilKw)
  ]);

  pub static ref TYPES: HashMap<&'static str, TokenKind> = HashMap::from([
    ("num", TokenKind::NumType),
    ("str", TokenKind::StrType),
    ("bool", TokenKind::BoolType)
  ]);
}

#[derive(PartialEq, Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub literal: Literal,
  pub pos: util::Position
}

#[derive(PartialEq)]
pub enum Literal {
  Num(f64),
  Str(String),
  Bool(bool),
  Nil
}

#[derive(PartialEq, Copy, Clone)]
pub enum TokenKind {
  Identifier,
  Number,
  String,

  StatEnd,

  Plus,
  Minus,
  Times,
  Divide,

  Modulo,

  LParen,
  RParen,

  Assign,
  Bang,

  And,
  Or,

  PlusAssign,
  MinusAssign,
  TimesAssign,
  DivideAssign,

  ModuloAssign,

  AndAssign,
  OrAssign,

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
  ElseKw,

  TrueKw,
  FalseKw,
  NilKw,

  NumType,
  StrType,
  BoolType,
}
