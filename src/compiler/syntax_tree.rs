mod error;
mod local_value;
mod unary_operator;
mod binary_operator;
mod syntax_tree_node;
mod syntax_tree;
pub(crate) mod generate_wasm;

use binary_operator::BinaryOperator;
use unary_operator::UnaryOperator;

use local_value::LocalValue;
pub(crate) use error::Error;
pub(crate) use syntax_tree_node::{FunctionNode, StatementNode, ExpressionNode, ModuleNode, Return, Node};
pub(crate) use syntax_tree::SyntaxTree;
