use std::collections::HashMap;

use super::{UnaryOperator, BinaryOperator, LocalValue};
use super::super::tokens::Token;

#[derive(Debug, PartialEq)]
pub(crate) struct Node<'a, T> {
    pub(crate) token: Token<'a>,
    pub(crate) entity: T,
}
impl<'a, T>  Node<'a, T> {
    pub(crate) fn new(entity: T, token: Token<'a>) -> Node<'a, T> {
        Node { token, entity }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ModuleNode<'a> {
    pub(crate) functions: HashMap<&'a str, Node<'a, FunctionNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Return<'a> {
    pub(crate) type_: &'a str,
    pub(crate) expression: Node<'a, ExpressionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) name: &'a str,
    pub(crate) arguments: Vec<LocalValue<'a>>,
    pub(crate) arguments_map: HashMap<&'a str, LocalValue<'a>>,
    pub(crate) local_values: HashMap<&'a str, LocalValue<'a>>,
    pub(crate) statements: Vec<StatementNode<'a>>,
    pub(crate) return_: Option<Node<'a, Return<'a>>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatementNode<'a> {
    Expression(Node<'a, ExpressionNode<'a>>),
    Assign(Node<'a, &'a str>, Node<'a, ExpressionNode<'a>>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExpressionNode<'a> {
    Identifier(&'a str),
    Value(&'a str),
    FunctionCall {
        name: &'a str,
        arguments: Vec<Node<'a, ExpressionNode<'a>>>,
    },
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
