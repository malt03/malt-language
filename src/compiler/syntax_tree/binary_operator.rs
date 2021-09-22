use super::super::tokens::TokenKind;

#[derive(Debug, PartialEq)]
pub(crate) enum BinaryOperator {
  Plus,
  Minus,
  Times,
  Divide,
}

impl From<&TokenKind> for BinaryOperator {
  fn from(kind: &TokenKind) -> Self {
    match kind {
      TokenKind::Plus => BinaryOperator::Plus,
      TokenKind::Minus => BinaryOperator::Minus,
      TokenKind::Times => BinaryOperator::Times,
      TokenKind::Divide => BinaryOperator::Divide,
      _ => panic!("unexpected")
    }
  }
}
