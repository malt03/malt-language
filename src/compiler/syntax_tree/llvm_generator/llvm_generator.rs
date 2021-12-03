use std::{collections::HashMap};

use super::error::{Result, Error};

use inkwell::{builder::Builder, context::Context, values::{IntValue, FunctionValue, BasicMetadataValueEnum}, types::BasicMetadataTypeEnum, module::Module};
use super::super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator, UnaryOperator, ModuleNode};

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

struct Scope<'ctx, 'a> {
    local_values: HashMap<&'a str, IntValue<'ctx>>
}

impl<'ctx> LLVMGenerator<'ctx> {
    pub(crate) fn new(context: &'ctx Context) -> LLVMGenerator<'ctx> {
        LLVMGenerator {
            context,
            builder: context.create_builder(),
        }
    }

    pub(crate) fn module<'a>(&self, node: &ModuleNode<'a>) -> Result<'a, Module<'ctx>> {
        let module = self.context.create_module("main");
        
        let functions: HashMap<_, _> = node.functions.iter().map(|function| {
            let name = function.name.value();
            let param_types = function.arguments.iter().map(|_| self.context.i64_type().into()).collect::<Vec<BasicMetadataTypeEnum>>();
            let ty = self.context.i64_type().fn_type(param_types.as_slice(), false);
            let val = module.add_function(name, ty, None);
            (name, val)
        }).collect();

        for function in &node.functions {
            self.function(function, &functions)?;
        }
        
        Ok(module)
    }

    pub(crate) fn function<'a>(&self, node: &FunctionNode<'a>, functions: &HashMap<&'a str, FunctionValue<'ctx>>) -> Result<'a, ()> {
        let val = *functions.get(node.name.value()).unwrap();
        let entry = self.context.append_basic_block(val, "entry");
        self.builder.position_at_end(entry);

        let local_values: HashMap<&'a str, IntValue<'ctx>> = val.get_param_iter().enumerate().map(|(i, arg)| {
            (node.arguments[i].name.value(), arg.into_int_value())
        }).collect();

        let mut scope = Scope { local_values };
        for statement in &node.statements {
            self.statement(statement, &mut scope, functions)?;
        }

        if let Some(expression) = node.return_expression.as_ref() {
            let value = self.expression(expression, &mut scope, functions)?;
            self.builder.build_return(Some(&value));
        }

        Ok(())
    }

    fn statement<'a>(&self, node: &StatementNode<'a>, scope: &mut Scope<'ctx, 'a>, functions: &HashMap<&'a str, FunctionValue<'ctx>>) -> Result<'a, ()> {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, scope, functions)?;
            }
            StatementNode::Assign { lhs, rhs } => {
                let expression = self.expression(rhs, scope, functions)?;
                scope.local_values.insert(lhs.name.value(), expression);
            },
        }
        Ok(())
    }

    fn expression<'a>(&self, node: &ExpressionNode<'a>, scope: &mut Scope<'ctx, 'a>, functions: &HashMap<&'a str, FunctionValue<'ctx>>) -> Result<'a, IntValue<'ctx>> {
        match node {
            ExpressionNode::Value(token) => {
                Ok(self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap())
            },
            ExpressionNode::Identifier(token) => {
                match scope.local_values.get(token.value()) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::value_not_found(token)),
                }
            },
            ExpressionNode::FunctionCall { token, arguments } => {
                match functions.get(token.value()) {
                    Some(value) => {
                        let arguments = arguments.iter().map(|expression| {
                            self.expression(expression, scope, functions).map(|v| v.into())
                        }).collect::<Result<Vec<BasicMetadataValueEnum>>>()?;
                        Ok(self.builder.build_call(*value, arguments.as_slice(), "calltmp").try_as_basic_value().left().unwrap().into_int_value())
                    },
                    None => Err(Error::function_not_found(token)),
                }
            },
            ExpressionNode::UnaryExpr { child, operator } => {
                match operator {
                    UnaryOperator::Minus => {
                        let child = self.expression(child, scope, functions)?;
                        Ok(self.builder.build_int_neg(child, "negtmp"))
                    },
                }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator } => {
                let lhs = self.expression(lhs, scope, functions)?;
                let rhs = self.expression(rhs, scope, functions)?;
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
