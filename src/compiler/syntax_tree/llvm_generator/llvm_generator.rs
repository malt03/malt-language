use std::{collections::HashMap};

use super::error::{Result, Error};

use inkwell::{builder::Builder, context::Context, values::{FunctionValue, BasicMetadataValueEnum, BasicValueEnum}, types::{BasicMetadataTypeEnum, BasicTypeEnum, BasicType, FunctionType}, module::Module};
use super::super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator, UnaryOperator, ModuleNode};
use super::super::super::tokens::Token;

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

struct Scope<'ctx, 'a> {
    local_values: HashMap<&'a str, BasicValueEnum<'ctx>>,
    functions: HashMap<&'a str, FunctionValue<'ctx>>,
}

impl<'ctx> LLVMGenerator<'ctx> {
    pub(crate) fn new(context: &'ctx Context) -> LLVMGenerator<'ctx> {
        LLVMGenerator {
            context,
            builder: context.create_builder(),
        }
    }

    fn type_to_metadata_type_enum<'a>(&self, token: &Token<'a>) -> Result<'a, BasicTypeEnum<'ctx>> {
        match token.value() {
            "Int" => Ok(self.context.i64_type().into()),
            "Double" => Ok(self.context.f64_type().into()),
            "Bool" => Ok(self.context.bool_type().into()),
            _ => Err(Error::type_not_found(token)),
        }
    }

    fn type_to_fn_type<'a>(&self, token: Option<&Token<'a>>, param_types: &[BasicMetadataTypeEnum<'ctx>]) -> Result<'a, FunctionType<'ctx>> {
        match token {
            Some(token) => Ok(self.type_to_metadata_type_enum(token)?.fn_type(param_types, false)),
            None => Ok(self.context.void_type().fn_type(param_types, false)),
        }
    }
    
    pub(crate) fn module<'a>(&self, node: &ModuleNode<'a>) -> Result<'a, Module<'ctx>> {
        let module = self.context.create_module("main");

        let functions: HashMap<_, _> = node.functions.iter().map(|function| {
            let name = function.name.value();
            let param_types = function.arguments.iter().map(|arg| {
                self.type_to_metadata_type_enum(&arg.typ).map(|t| t.into())
            }).collect::<Result<Vec<BasicMetadataTypeEnum>>>()?;
            let ty = self.type_to_fn_type(function.return_type.as_ref(), param_types.as_slice())?;
            let val = module.add_function(name, ty, None);
            Ok((name, val))
        }).collect::<Result<HashMap<_, _>>>()?;

        for function in &node.functions {
            self.function(function, &functions)?;
        }
        
        Ok(module)
    }

    pub(crate) fn function<'a>(&self, node: &FunctionNode<'a>, functions: &HashMap<&'a str, FunctionValue<'ctx>>) -> Result<'a, ()> {
        let val = *functions.get(node.name.value()).unwrap();
        let entry = self.context.append_basic_block(val, "entry");
        self.builder.position_at_end(entry);

        let local_values: HashMap<&'a str, BasicValueEnum<'ctx>> = val.get_param_iter().enumerate().map(|(i, arg)| {
            (node.arguments[i].name.value(), arg.into())
        }).collect();

        let mut scope = Scope { local_values, functions: functions.clone() };
        for statement in &node.statements {
            self.statement(statement, &mut scope)?;
        }

        if let Some(return_type_token) = node.return_type.as_ref() {
            let return_type = self.type_to_metadata_type_enum(return_type_token)?;
            match node.ret.as_ref() {
                Some(ret) => {
                    let value = self.expression(&ret.expression, Some(return_type), &mut scope)?;
                    self.builder.build_return(Some(&value));
                },
                None => {
                    return Err(Error::unexpected_type(return_type_token.value(), "Void", &node.close));
                },
            }
        } else if let Some(ret) = node.ret.as_ref() {
            return Err(Error::unexpected_type("Void", ret.token.value(), &ret.token));
        }
        
        Ok(())
    }

    fn statement<'a>(&self, node: &StatementNode<'a>, scope: &mut Scope<'ctx, 'a>) -> Result<'a, ()> {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, None, scope)?;
            }
            StatementNode::Assign { lhs, rhs } => {
                let typ = self.type_to_metadata_type_enum(&lhs.typ)?;
                let expression = self.expression(rhs, Some(typ), scope)?;
                scope.local_values.insert(lhs.name.value(), expression);
            },
        }
        Ok(())
    }

    fn const_from_string<'a>(&self, token: &Token<'a>, expected_type: Option<BasicTypeEnum<'ctx>>) -> BasicValueEnum<'ctx> {
        if let Some(expected_type) = expected_type {
            match expected_type {
                BasicTypeEnum::ArrayType(_) => unimplemented!(),
                BasicTypeEnum::FloatType(t) => t.const_float_from_string(token.value()).into(),
                BasicTypeEnum::IntType(t) => t.const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into(),
                BasicTypeEnum::PointerType(_) => unimplemented!(),
                BasicTypeEnum::StructType(_) => unimplemented!(),
                BasicTypeEnum::VectorType(_) => unimplemented!(),
            }
        } else {
            self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into()
        }
    }

    fn expression<'a>(
        &self,
        node: &ExpressionNode<'a>,
        expected_type: Option<BasicTypeEnum<'ctx>>,
        scope: &mut Scope<'ctx, 'a>,
    ) -> Result<'a, BasicValueEnum<'ctx>> {
        match node {
            ExpressionNode::Value(token) => Ok(self.const_from_string(token, expected_type)),
            ExpressionNode::Identifier(token) => {
                match scope.local_values.get(token.value()) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::value_not_found(token)),
                }
            },
            ExpressionNode::FunctionCall { token, arguments } => {
                let function = scope.functions.get(token.value()).map(|v| v.clone());
                if let Some(function) = function {
                    let param_types = function.get_type().get_param_types();
                    if param_types.len() != arguments.len() {
                        return Err(Error::unexpected_arguments_length(param_types.len(), arguments.len(), token))
                    }

                    let arguments = arguments.iter().enumerate().map(|(i, expression)| {
                        let param_type = param_types[i];
                        self.expression(expression, Some(param_type), scope).map(|v| v.into())
                    }).collect::<Result<Vec<BasicMetadataValueEnum>>>()?;
                    Ok(self.builder.build_call(function, arguments.as_slice(), "calltmp").try_as_basic_value().left().unwrap().into_int_value().into())
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
