use super::super::tokens::TokenKind;

#[derive(Debug, PartialEq)]
pub(crate) enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl From<&TokenKind> for BinaryOperator {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Plus => BinaryOperator::Plus,
            TokenKind::Minus => BinaryOperator::Minus,
            TokenKind::Multiply => BinaryOperator::Multiply,
            TokenKind::Divide => BinaryOperator::Divide,
            _ => panic!("unexpected")
        }
    }
}
