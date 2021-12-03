use super::super::{PeekableTokens, tokens::Token};
use super::{UnaryOperator, BinaryOperator, LocalValue};

#[derive(Debug, PartialEq)]
pub(crate) struct Node<'a, Kind> {
    pub(crate) kind: Kind,
    pub(crate) cursor: usize,
    pub(crate) text: &'a str,
}

impl<'a, Kind> Node<'a, Kind> {
    pub(crate) fn new(kind: Kind, cursor: usize, text: &'a str) -> Node<'a, Kind> {
        Node { kind: kind, cursor, text }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) local_values: Vec<LocalValue<'a>>,
    pub(crate) statements: Vec<Node<'a, StatementNode<'a>>>,
    pub(crate) return_statement: Option<Node<'a, ExpressionNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatementNode<'a> {
    Expression(Node<'a, ExpressionNode<'a>>),
    Assign(&'a str, Node<'a, ExpressionNode<'a>>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExpressionNode<'a> {
    Identifier(&'a str),
    Value(&'a str),
    UnaryExpr {
        child: Box<Node<'a, ExpressionNode<'a>>>,
        operator: UnaryOperator,
    },
    BinaryExpr {
        lhs: Box<Node<'a, ExpressionNode<'a>>>,
        rhs: Box<Node<'a, ExpressionNode<'a>>>,
        operator: BinaryOperator,
    },
}
