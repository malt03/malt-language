#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TokenKind {
  Identifier,
  Number,
  OpenParen,
  CloseParen,
  Plus,
  Minus,
  Times,
  Divide,
  Assign,
  Equal,
  Greater,
  GreaterOrEqual,
  Less,
  LessOrEqual,
  NewLine,
  EOF,
}
