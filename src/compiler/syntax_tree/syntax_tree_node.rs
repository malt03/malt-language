use super::{UnaryOperator, BinaryOperator};
use super::super::tokens::Token;

#[derive(Debug, PartialEq)]
pub(crate) struct ModuleNode<'a> {
    pub(crate) functions: Vec<FunctionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) name: Token<'a>,
    pub(crate) arguments: Vec<ValueDefinitionNode<'a>>,
    pub(crate) return_type: Option<Token<'a>>,
    pub(crate) statements: Vec<StatementNode<'a>>,
    pub(crate) ret: Option<ReturnNode<'a>>,
    pub(crate) close: Token<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ReturnNode<'a> {
    pub(crate) token: Token<'a>,
    pub(crate) expression: ExpressionNode<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ValueDefinitionNode<'a> {
    pub(crate) name: Token<'a>,
    pub(crate) typ: Token<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatementNode<'a> {
    Expression(ExpressionNode<'a>),
    Assign { lhs: ValueDefinitionNode<'a>, rhs: ExpressionNode<'a> },
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExpressionNode<'a> {
    Identifier(Token<'a>),
    Int(Token<'a>),
    Double(Token<'a>),
    Bool(bool, Token<'a>),
    FunctionCall {
        token: Token<'a>,
        arguments: Vec<CallArgumentNode<'a>>,
    },
    UnaryExpr {
        token: Token<'a>,
        child: Box<ExpressionNode<'a>>,
        operator: UnaryOperator,
    },
    BinaryExpr {
        token: Token<'a>,
        lhs: Box<ExpressionNode<'a>>,
        rhs: Box<ExpressionNode<'a>>,
        operator: BinaryOperator,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallArgumentNode<'a> {
    pub(crate) label: Token<'a>,
    pub(crate) value: ExpressionNode<'a>
}
