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


#[derive(Debug, PartialEq)]
pub(crate) enum CompareOperator {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

impl From<&TokenKind> for CompareOperator {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Equal => CompareOperator::Equal,
            TokenKind::NotEqual => CompareOperator::NotEqual,
            TokenKind::Greater => CompareOperator::Greater,
            TokenKind::GreaterOrEqual => CompareOperator::GreaterOrEqual,
            TokenKind::Less => CompareOperator::Less,
            TokenKind::LessOrEqual => CompareOperator::LessOrEqual,
            _ => panic!("unexpected")
        }
    }
}
