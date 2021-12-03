use std::{collections::HashMap};

use super::error::{Result, Error};

use inkwell::{builder::Builder, context::Context, module::Module, values::{IntValue, FunctionValue}};
use super::super::{Node, ExpressionNode, FunctionNode, StatementNode, BinaryOperator, UnaryOperator};

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    builder: Builder<'ctx>,
}

struct Scope<'ctx, 'a> {
    local_values: HashMap<&'a str, IntValue<'ctx>>
}

impl<'ctx, 'a> Scope<'ctx, 'a> {
    fn new() -> Scope<'ctx, 'a> {
        Scope { local_values: HashMap::new() }
    }
}

impl<'ctx> LLVMGenerator<'ctx> {
    pub(crate) fn new(context: &'ctx Context) -> LLVMGenerator<'ctx> {
        LLVMGenerator {
            context,
            module: context.create_module("main"),
            builder: context.create_builder(),
        }
    }

    pub(crate) fn function<'a>(&self, node: &Node<'a, FunctionNode<'a>>) -> Result<'a, FunctionValue<'ctx>> {
        let fn_type = self.context.i64_type().fn_type(&[], false);
        let fn_val = self.module.add_function("main", fn_type, None);

        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);

        let mut scope = Scope::new();
        for statement in &node.kind.statements {
            self.statement(statement, &mut scope)?;
        }

        if let Some(expression) = node.kind.return_statement.as_ref() {
            let value = self.expression(expression, &mut scope)?;
            self.builder.build_return(Some(&value));
        }

        Ok(fn_val)
    }

    fn statement<'a>(&self, node: &Node<'a, StatementNode<'a>>, scope: &mut Scope<'ctx, 'a>) -> Result<'a, ()> {
        match &node.kind {
            StatementNode::Expression(expression) => {
                self.expression(expression, scope)?;
            }
            StatementNode::Assign(name, expression) => {
                let expression = self.expression(expression, scope)?;
                scope.local_values.insert(name, expression);
            },
        }
        Ok(())
    }

    fn expression<'a>(&self, node: &Node<'a, ExpressionNode<'a>>, scope: &mut Scope<'ctx, 'a>) -> Result<'a, IntValue<'ctx>> {
        match &node.kind {
            ExpressionNode::Value(value) => {
                Ok(self.context.i64_type().const_int_from_string(value, inkwell::types::StringRadix::Decimal).unwrap())
            },
            ExpressionNode::Identifier(name) => {
                match scope.local_values.get(name) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::ValueNotFound { name, cursor: node.cursor, text: node.text }),
                }
            },
            ExpressionNode::UnaryExpr { child, operator } => {
                match operator {
                    UnaryOperator::Minus => {
                        let child = self.expression(child, scope)?;
                        Ok(self.builder.build_int_neg(child, "negtmp"))
                    },
                }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator } => {
                let lhs = self.expression(lhs, scope)?;
                let rhs = self.expression(rhs, scope)?;
                let value = match operator {
                    BinaryOperator::Plus => self.builder.build_int_add(lhs, rhs, "addtmp"),
                    BinaryOperator::Minus => self.builder.build_int_sub(lhs, rhs, "subtmp"),
                    BinaryOperator::Multiply => self.builder.build_int_mul(lhs, rhs, "multmp"),
                    BinaryOperator::Divide => self.builder.build_int_signed_div(lhs, rhs, "divtmp"),
                };
                Ok(value)
            },
        }
    }
}
