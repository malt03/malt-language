use super::{UnaryOperator, BinaryOperator};

#[derive(Debug, PartialEq)]
pub(crate) enum SyntaxTreeNode<'a> {
    Value(&'a str),
    UnaryExpr {
        child: Box<SyntaxTreeNode<'a>>,
        operator: UnaryOperator,
    },
    BinaryExpr {
        lhs: Box<SyntaxTreeNode<'a>>,
        rhs: Box<SyntaxTreeNode<'a>>,
        operator: BinaryOperator,
    },
}
