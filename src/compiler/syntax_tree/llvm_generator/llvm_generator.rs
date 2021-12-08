use std::{collections::HashMap};

use super::{error::{Result, Error}, typ::{TypeMap, Function, Type}};

use inkwell::{builder::Builder, context::Context, values::{BasicMetadataValueEnum, BasicValueEnum}, module::Module};
use super::super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator, UnaryOperator, ModuleNode};
use super::super::super::tokens::Token;

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

struct Scope<'a, 'module, 'ctx> {
    local_values: &'module mut HashMap<&'a str, BasicValueEnum<'ctx>>,
    functions: &'module HashMap<&'a str, Function<'a, 'ctx>>,
    type_map: &'module TypeMap<'a>,
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
        let type_map = TypeMap::new();

        let functions: HashMap<_, _> = node.functions.iter().map(|function| {
            let name = function.name.value();
            let function = Function::new(&type_map, function, self.context, &module)?;
            Ok((name, function))
        }).collect::<Result<HashMap<_, _>>>()?;

        let scope = Scope { local_values: &mut HashMap::new(), functions: &functions, type_map: &type_map };

        for function in &node.functions {
            self.function(function, &scope)?;
        }
        
        Ok(module)
    }

    fn function<'a, 'module>(&self, node: &FunctionNode<'a>, scope: &Scope<'a, 'module, 'ctx>) -> Result<'a, ()> {
        let function = scope.functions.get(node.name.value()).unwrap();

        let entry = self.context.append_basic_block(function.val, "entry");
        self.builder.position_at_end(entry);

        let mut local_values: HashMap<&'a str, BasicValueEnum<'ctx>> = function.val.get_param_iter().enumerate().map(|(i, arg)| {
            (node.arguments[i].name.value(), arg.into())
        }).collect();

        let mut scope = Scope { local_values: &mut local_values, functions: scope.functions, type_map: scope.type_map };
        for statement in &node.statements {
            self.statement(statement, &mut scope)?;
        }

        if let Some(return_type) = function.return_type {
            match node.ret.as_ref() {
                Some(ret) => {
                    let value = self.expression(&ret.expression, Some(return_type), &mut scope)?;
                    self.builder.build_return(Some(&value));
                },
                None => {
                    return Err(Error::unexpected_type(return_type.to_str(), "Void", &node.close));
                },
            }
        } else if let Some(ret) = node.ret.as_ref() {
            return Err(Error::unexpected_type("Void", ret.token.value(), &ret.token));
        }
        
        Ok(())
    }

    fn statement<'a, 'module>(&self, node: &StatementNode<'a>, scope: &mut Scope<'a, 'module, 'ctx>) -> Result<'a, ()> {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, None, scope)?;
            }
            StatementNode::Assign { lhs, rhs } => {
                let typ = scope.type_map.get(&lhs.typ)?;
                let expression = self.expression(rhs, Some(typ), scope)?;
                scope.local_values.insert(lhs.name.value(), expression);
            },
        }
        Ok(())
    }

    fn const_from_string<'a>(&self, token: &Token<'a>, expected_type: Option<Type>) -> Result<'a, BasicValueEnum<'ctx>> {
        if let Some(expected_type) = expected_type {
            match expected_type {
                Type::Int => Ok(self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into()),
                Type::Double => Ok(self.context.f64_type().const_float_from_string(token.value()).into()),
                Type::Bool => {
                    match token.value() {
                        "true" => Ok(self.context.bool_type().const_int(1, false).into()),
                        "false" => Ok(self.context.bool_type().const_int(0, false).into()),
                        _ => Err(Error::unexpected_type("Bool", token.value(), token)),
                    }
                },
            }
        } else {
            Ok(self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into())
        }
    }

    fn expression<'a, 'module>(
        &self,
        node: &ExpressionNode<'a>,
        expected_type: Option<Type>,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, BasicValueEnum<'ctx>> {
        match node {
            ExpressionNode::Value(token) => Ok(self.const_from_string(token, expected_type)?),
            ExpressionNode::Identifier(token) => {
                match scope.local_values.get(token.value()) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::value_not_found(token)),
                }
            },
            ExpressionNode::FunctionCall { token, arguments } => {
                let function = scope.functions.get(token.value()).map(|v| v.clone());
                if let Some(function) = function {
                    if function.arguments.len() != arguments.len() {
                        return Err(Error::unexpected_arguments_length(function.arguments.len(), arguments.len(), token))
                    }

                    let arguments = arguments.iter().enumerate().map(|(i, expression)| {
                        let (_, expected_type) = function.arguments[i];
                        self.expression(expression, Some(expected_type), scope).map(|v| v.into())
                    }).collect::<Result<Vec<BasicMetadataValueEnum>>>()?;

                    Ok(function.build_call(&self.builder, arguments.as_slice()))
                } else { Err(Error::function_not_found(token)) }
            },
            ExpressionNode::UnaryExpr { child, operator, token } => {
                match operator {
                    UnaryOperator::Minus => {
                        let child = self.expression(child, expected_type, scope)?;
                        if child.is_float_value() {
                            Ok(self.builder.build_float_neg(child.into_float_value(), "negtmp").into())
                        } else if child.is_int_value() {
                            Ok(self.builder.build_int_neg(child.into_int_value(), "negtmp").into())
                        } else {
                            Err(Error::cannot_apply_operator(token))
                        }
                    },
                }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator } => {
                let lhs = self.expression(lhs, expected_type, scope)?;
                let rhs = self.expression(rhs, expected_type, scope)?;
                let value = match operator {
                    BinaryOperator::Plus => self.builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "addtmp").into(),
                    BinaryOperator::Minus => self.builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "subtmp").into(),
                    BinaryOperator::Multiply => self.builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "multmp").into(),
                    BinaryOperator::Divide => self.builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "divtmp").into(),
                };
                Ok(value)
            },
        }
    }
}
