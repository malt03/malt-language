use std::{collections::HashMap};

use inkwell::{builder::Builder, context::Context, module::Module, values::{IntValue, FunctionValue}};
use super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator};

pub(crate) struct LLVM<'ctx> {
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

impl<'ctx> LLVM<'ctx> {
    pub(crate) fn new(context: &'ctx Context) -> LLVM<'ctx> {
        LLVM {
            context,
            module: context.create_module("main"),
            builder: context.create_builder(),
        }
    }

    pub(crate) fn function<'a>(&self, node: &FunctionNode<'a>) -> FunctionValue<'ctx> {
        let fn_type = self.context.i64_type().fn_type(&[], false);
        let fn_val = self.module.add_function("main", fn_type, None);

        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);

        let mut scope = Scope::new();
        for statement in &node.statements {
            self.statement(statement, &mut scope);
        }

        if let Some(expression) = node.return_statement.as_ref() {
            let value = self.expression(expression, &mut scope);
            self.builder.build_return(Some(&value));
        }

        fn_val
    }

    fn statement<'a>(&self, node: &StatementNode<'a>, scope: &mut Scope<'ctx, 'a>) {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, scope);
            }
            StatementNode::Assign(name, expression) => {
                let expression = self.expression(expression, scope);
                scope.local_values.insert(name, expression);
            },
        }
    }

    fn expression<'a>(&self, node: &ExpressionNode<'a>, scope: &mut Scope<'ctx, 'a>) -> IntValue<'ctx> {
        match node {
            ExpressionNode::Value(value) => {
                self.context.i64_type().const_int_from_string(value, inkwell::types::StringRadix::Decimal).unwrap()
            },
            ExpressionNode::Identifier(name) => {
                match scope.local_values.get(name) {
                    Some(value) => value.clone(),
                    None => unimplemented!(),
                }
            },
            ExpressionNode::UnaryExpr { child, operator } => {
                unimplemented!()
                // match operator {
                //     UnaryOperator::Minus => {
                //         writer.write_all(b"(i32.sub (i32.const 0)")?;
                //         self.write_expression(writer, child)?;
                //         writer.write_all(b")")?;
                //     },
                // }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator } => {
                let lhs = self.expression(lhs, scope);
                let rhs = self.expression(rhs, scope);
                match operator {
                    BinaryOperator::Plus => self.builder.build_int_add(lhs, rhs, "addtmp"),
                    BinaryOperator::Minus => self.builder.build_int_sub(lhs, rhs, "subtmp"),
                    BinaryOperator::Multiply => self.builder.build_int_mul(lhs, rhs, "multmp"),
                    BinaryOperator::Divide => self.builder.build_int_signed_div(lhs, rhs, "divtmp"),
                }
            },
        }
    }
}
