use std::{collections::HashMap};

use super::{error::{Result, Error}, typ::{TypeMap, Function, Type, VoidableType}};

use inkwell::{builder::Builder, context::Context, values::{BasicMetadataValueEnum, BasicValueEnum}, module::Module, IntPredicate, FloatPredicate};
use super::super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator, CompareOperator, UnaryOperator, ModuleNode};
use super::super::super::tokens::Token;

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

struct Scope<'a, 'module, 'ctx> {
    local_values: &'module mut HashMap<&'a str, (Type, BasicValueEnum<'ctx>)>,
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

        let mut local_values: HashMap<&'a str, (Type, BasicValueEnum<'ctx>)> = function.val.get_param_iter().enumerate().map(|(i, arg)| {
            let (name, typ) = function.arguments[i];
            (name, (typ, arg.into()))
        }).collect();

        let mut scope = Scope { local_values: &mut local_values, functions: scope.functions, type_map: scope.type_map };
        for statement in &node.block.statements {
            self.statement(statement, &mut scope)?;
        }

        if let VoidableType::Type(return_type) = function.return_type {
            match node.block.ret.as_ref() {
                Some(ret) => {
                    let (_, value) = self.expression(&ret.expression, Some(return_type), &mut scope)?.unwrap();
                    self.builder.build_return(Some(&value));
                },
                None => {
                    return Err(Error::unexpected_type(return_type.to_str(), "Void", &node.block.close));
                },
            }
        } else if let Some(ret) = node.block.ret.as_ref() {
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
                let expected_type = scope.type_map.get(&lhs.typ)?;
                let (_, expression) = self.expression(rhs, Some(expected_type), scope)?.unwrap();
                scope.local_values.insert(lhs.name.value(), (expected_type, expression));
            },
        }
        Ok(())
    }

    fn validate_expected_type<'a>(expected_type: Option<Type>, typ: VoidableType, token: &Token<'a>) -> Result<'a, ()> {
        if let Some(expected_type) = expected_type {
            if let VoidableType::Type(typ) = typ {
                if typ != expected_type {
                    return Err(Error::unexpected_type(expected_type.to_str(), token.value(), token));
                }
            } else {
                return Err(Error::unexpected_type(expected_type.to_str(), token.value(), token));
            }
        }
        Ok(())
    }

    fn expression<'a, 'module>(
        &self,
        node: &ExpressionNode<'a>,
        expected_type: Option<Type>,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, Option<(Type, BasicValueEnum<'ctx>)>> {
        match node {
            ExpressionNode::Bool(value, token) => {
                LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Bool), token)?;
                Ok(Some((Type::Bool, self.context.bool_type().const_int(if *value { 1 } else { 0 }, false).into())))
            },
            ExpressionNode::Int(token) => {
                if let Some(expected_type) = expected_type {
                    match expected_type {
                        Type::Int => Ok(Some((
                            Type::Int,
                            self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into(),
                        ))),
                        Type::Double => Ok(Some((
                            Type::Double,
                            self.context.f64_type().const_float_from_string(token.value()).into(),
                        ))),
                        _ => Err(Error::unexpected_type(expected_type.to_str(), token.value(), token))
                    }
                } else {
                    Ok(Some((
                        Type::Int,
                        self.context.i64_type().const_int_from_string(token.value(), inkwell::types::StringRadix::Decimal).unwrap().into()
                    )))
                }
            },
            ExpressionNode::Double(token) => {
                LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Double), token)?;
                Ok(Some((
                    Type::Double,
                    self.context.f64_type().const_float_from_string(token.value()).into()
                )))
            },
            ExpressionNode::Identifier(token) => {
                match scope.local_values.get(token.value()) {
                    Some((typ, value)) => {
                        LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(*typ), token)?;
                        Ok(Some((*typ, value.clone())))
                    },
                    None => Err(Error::value_not_found(token)),
                }
            },
            ExpressionNode::FunctionCall { token, arguments } => {
                let function = scope.functions.get(token.value()).map(|v| v.clone());
                if let Some(function) = function {
                    LLVMGenerator::validate_expected_type(expected_type, function.return_type, token)?;

                    if function.arguments.len() != arguments.len() {
                        return Err(Error::unexpected_arguments_length(function.arguments.len(), arguments.len(), token))
                    }

                    let arguments = arguments.iter().enumerate().map(|(i, argument)| {
                        let (name, expected_type) = function.arguments[i];
                        if name != argument.label.value() {
                            return Err(Error::unexpected_label(name, &argument.label))
                        }
                        let (_, v) = self.expression(&argument.value, Some(expected_type), scope)?.unwrap();
                        Ok(v.into())
                    }).collect::<Result<Vec<BasicMetadataValueEnum>>>()?;

                    Ok(function.build_call(&self.builder, arguments.as_slice()))
                } else { Err(Error::function_not_found(token)) }
            },
            ExpressionNode::UnaryExpr { child, operator, token } => {
                match operator {
                    UnaryOperator::Minus => {
                        let (typ, child) = self.expression(child, expected_type, scope)?
                            .ok_or(Error::unexpected_type("Int, Double", "Void", token))?;
                        let typ = expected_type.unwrap_or(typ);
                        match typ {
                            Type::Int => Ok(Some((
                                Type::Int,
                                self.builder.build_int_neg(child.into_int_value(), "negtmp").into(),
                            ))),
                            Type::Double => Ok(Some((
                                Type::Double,
                                self.builder.build_float_neg(child.into_float_value(), "negtmp").into(),
                            ))),
                            _ => Err(Error::unexpected_type(typ.to_str(), token.value(), token))
                        }
                    },
                }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator, token } => {
                let (lhs_type, lhs_value) = self.expression(lhs, expected_type, scope)?
                    .ok_or(Error::unexpected_type("Comparable", "Void", token))?;
                let (_, rhs_value) = self.expression(rhs, Some(lhs_type), scope)?.unwrap();
                match lhs_type {
                    Type::Int => {
                        let value = match operator {
                            BinaryOperator::Plus => self.builder.build_int_add(lhs_value.into_int_value(), rhs_value.into_int_value(), "addtmp").into(),
                            BinaryOperator::Minus => self.builder.build_int_sub(lhs_value.into_int_value(), rhs_value.into_int_value(), "subtmp").into(),
                            BinaryOperator::Multiply => self.builder.build_int_mul(lhs_value.into_int_value(), rhs_value.into_int_value(), "multmp").into(),
                            BinaryOperator::Divide => self.builder.build_int_signed_div(lhs_value.into_int_value(), rhs_value.into_int_value(), "divtmp").into(),
                        };
                        Ok(Some((Type::Int, value)))
                    },
                    Type::Double => {
                        let value = match operator {
                            BinaryOperator::Plus => self.builder.build_float_add(lhs_value.into_float_value(), rhs_value.into_float_value(), "addtmp").into(),
                            BinaryOperator::Minus => self.builder.build_float_sub(lhs_value.into_float_value(), rhs_value.into_float_value(), "subtmp").into(),
                            BinaryOperator::Multiply => self.builder.build_float_mul(lhs_value.into_float_value(), rhs_value.into_float_value(), "multmp").into(),
                            BinaryOperator::Divide => self.builder.build_float_div(lhs_value.into_float_value(), rhs_value.into_float_value(), "divtmp").into(),
                        };
                        Ok(Some((Type::Double, value)))
                    },
                    _ => Err(Error::unexpected_type(lhs_type.to_str(), token.value(), token))
                }
            },
            ExpressionNode::CompareExpr { token, lhs, rhs, operator } => {
                LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Bool), token)?;
                
                let (lhs_type, lhs_value) = self.expression(lhs, None, scope)?
                    .ok_or(Error::unexpected_type("Comparable", "Void", token))?;
                let (_, rhs_value) = self.expression(rhs, Some(lhs_type), scope)?.unwrap();
                
                match lhs_type {
                    Type::Int => {
                        let value = self.builder.build_int_compare(operator.into(), lhs_value.into_int_value(), rhs_value.into_int_value(), "cmptmp");
                        Ok(Some((Type::Bool, value.into())))
                    },
                    Type::Double => {
                        let value = self.builder.build_float_compare(operator.into(), lhs_value.into_float_value(), rhs_value.into_float_value(), "cmptmp");
                        Ok(Some((Type::Bool, value.into())))
                    },
                    _ => Err(Error::unexpected_type(lhs_type.to_str(), token.value(), token))
                }
            },
        }
    }
}

impl Into<IntPredicate> for &CompareOperator {
    fn into(self) -> IntPredicate {
        match self {
            CompareOperator::Equal => IntPredicate::EQ,
            CompareOperator::NotEqual => IntPredicate::NE,
            CompareOperator::Greater => IntPredicate::SGT,
            CompareOperator::GreaterOrEqual => IntPredicate::SGE,
            CompareOperator::Less => IntPredicate::SLT,
            CompareOperator::LessOrEqual => IntPredicate::SLE,
        }
    }
}

impl Into<FloatPredicate> for &CompareOperator {
    fn into(self) -> FloatPredicate {
        match self {
            CompareOperator::Equal => FloatPredicate::OEQ,
            CompareOperator::NotEqual => FloatPredicate::ONE,
            CompareOperator::Greater => FloatPredicate::OGT,
            CompareOperator::GreaterOrEqual => FloatPredicate::OGE,
            CompareOperator::Less => FloatPredicate::OLT,
            CompareOperator::LessOrEqual => FloatPredicate::OLE,
        }
    }
}
