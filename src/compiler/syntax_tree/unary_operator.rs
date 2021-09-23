use super::super::tokens::TokenKind;

#[derive(Debug, PartialEq)]
pub(crate) enum UnaryOperator {
    Minus,
}

impl From<&TokenKind> for UnaryOperator {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Minus => UnaryOperator::Minus,
            _ => panic!("unexpected")
        }
    }
}
