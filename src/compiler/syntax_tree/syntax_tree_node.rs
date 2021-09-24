use super::{UnaryOperator, BinaryOperator, LocalValue};

#[derive(Debug, PartialEq)]
pub(crate) struct ModuleNode<'a> {
    pub(crate) functions: Vec<FunctionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) name: &'a str,
    pub(crate) arguments: Vec<LocalValue<'a>>,
    pub(crate) local_values: Vec<LocalValue<'a>>,
    pub(crate) statements: Vec<StatementNode<'a>>,
    pub(crate) return_statement: Option<ExpressionNode<'a>>,
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
