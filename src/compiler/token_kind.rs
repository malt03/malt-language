#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}
