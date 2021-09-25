use std::collections::HashMap;

use super::{UnaryOperator, BinaryOperator, LocalValue};

#[derive(Debug, PartialEq)]
pub(crate) struct ModuleNode<'a> {
    pub(crate) functions: HashMap<&'a str, FunctionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Return<'a> {
    pub(crate) type_: &'a str,
    pub(crate) expression: ExpressionNode<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) name: &'a str,
    pub(crate) arguments: Vec<LocalValue<'a>>,
    pub(crate) arguments_map: HashMap<&'a str, LocalValue<'a>>,
    pub(crate) local_values: HashMap<&'a str, LocalValue<'a>>,
    pub(crate) statements: Vec<StatementNode<'a>>,
    pub(crate) return_: Option<Return<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatementNode<'a> {
    Expression(ExpressionNode<'a>),
    Assign(&'a str, ExpressionNode<'a>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExpressionNode<'a> {
    Identifier(&'a str),
    Value(&'a str),
    FunctionCall {
        name: &'a str,
        arguments: Vec<ExpressionNode<'a>>,
    },
    UnaryExpr {
        child: Box<ExpressionNode<'a>>,
        operator: UnaryOperator,
    },
    BinaryExpr {
        lhs: Box<ExpressionNode<'a>>,
        rhs: Box<ExpressionNode<'a>>,
        operator: BinaryOperator,
    },
}
