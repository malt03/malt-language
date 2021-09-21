#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenKind<'a> {
  Identifier(&'a str),
  Number(&'a str),
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
