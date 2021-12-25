use super::{super::tokens::Token, unary_operator::UnaryOperator, binary_operator::{BinaryOperator, CompareOperator}};

#[derive(Debug, PartialEq)]
pub(crate) struct ModuleNode<'a> {
    pub(crate) functions: Vec<FunctionNode<'a>>,
    pub(crate) structs: Vec<StructNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionNode<'a> {
    pub(crate) name: Token<'a>,
    pub(crate) arguments: Vec<ValueDefinitionNode<'a>>,
    pub(crate) name_with_arguments: String,
    pub(crate) return_type: Option<Token<'a>>,
    pub(crate) block: BlockNode<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct StructNode<'a> {
    pub(crate) name: Token<'a>,
    pub(crate) properties: Vec<ValueDefinitionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BlockNode<'a> {
    pub(crate) statements: Vec<StatementNode<'a>>,
    pub(crate) ret: Option<ReturnNode<'a>>,
    pub(crate) close: Token<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ReturnNode<'a> {
    pub(crate) token: Token<'a>,
    pub(crate) expression: Option<ExpressionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ValueDefinitionNode<'a> {
    pub(crate) name: Token<'a>,
    pub(crate) type_: Token<'a>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatementNode<'a> {
    Expression(ExpressionNode<'a>),
    Assign { name: Token<'a>, type_: Option<Token<'a>>, rhs: ExpressionNode<'a> },
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
        name_with_arguments: String,
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
    CompareExpr {
        token: Token<'a>,
        lhs: Box<ExpressionNode<'a>>,
        rhs: Box<ExpressionNode<'a>>,
        operator: CompareOperator,
    },
    IfBranch {
        token: Token<'a>,
        if_branches: Vec<(Box<ExpressionNode<'a>>, Box<BlockNode<'a>>)>,
        else_branch: Option<Box<BlockNode<'a>>>,
    },
    StructProperty {
        instance: Token<'a>,
        property: Token<'a>,
    },
    StructConstruction {
        type_: Token<'a>,
        arguments: Vec<CallArgumentNode<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallArgumentNode<'a> {
    pub(crate) label: Token<'a>,
    pub(crate) value: ExpressionNode<'a>
}
