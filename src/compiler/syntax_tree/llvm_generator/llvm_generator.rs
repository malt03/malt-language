use std::collections::HashMap;

use crate::compiler::syntax_tree::BlockNode;

use super::{error::{Result, Error}, typ::{Function, Type, VoidableType, ExpectedType}, scope::{ScopeValues, Scope}};

use inkwell::{builder::Builder, context::Context, values::{BasicMetadataValueEnum, BasicValueEnum, BasicValue}, module::Module, IntPredicate, FloatPredicate, basic_block::BasicBlock};
use super::super::{ExpressionNode, FunctionNode, StatementNode, BinaryOperator, CompareOperator, UnaryOperator, ModuleNode};
use super::super::super::tokens::Token;

pub(crate) struct LLVMGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
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

        let scope = Scope::module(ScopeValues::empty());

        let functions: HashMap<_, _> = node.functions.iter().map(|function| {
            let name = function.name.value();
            let function = Function::new(&scope, function, self.context, &module)?;
            Ok((name, function))
        }).collect::<Result<HashMap<_, _>>>()?;

        let scope = Scope::module(ScopeValues::new(HashMap::new(), functions));
        for function in &node.functions {
            self.function(function, &scope)?;
        }
        
        Ok(module)
    }

    fn function<'a, 'module>(&self, node: &FunctionNode<'a>, scope: &Scope<'a, 'module, 'ctx>) -> Result<'a, ()> {
        let function = scope.get_function(&node.name)?;

        let local_values: HashMap<&'a str, (Type, BasicValueEnum<'ctx>)> = function.val.get_param_iter().enumerate().map(|(i, arg)| {
            let (name, typ) = function.arguments[i];
            (name, (typ, arg.into()))
        }).collect();

        let scope = Scope::function(
            function,
            ScopeValues::new(local_values, HashMap::new()),
            scope,
        );

        let block = self.context.append_basic_block(scope.current_function.unwrap().val, "block");
        self.builder.position_at_end(block);
        let (has_return, _) = self.block(&node.block, ExpectedType::None, function.return_type, &scope)?;

        if !has_return && function.return_type.is_type() {
            Err(Error::unexpected_type(function.return_type.to_str(), "Void", &node.block.close))
        } else {
            Ok(())
        }
    }

    fn statement<'a, 'module>(&self, node: &StatementNode<'a>, expected_type: ExpectedType, scope: &mut Scope<'a, 'module, 'ctx>) -> Result<'a, Option<(Type, BasicValueEnum<'ctx>)>> {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, expected_type, scope)
            }
            StatementNode::Assign { name, typ, rhs } => {
                match expected_type {
                    ExpectedType::Type(expected_type) => Err(Error::unexpected_type(expected_type.to_str(), "assign", name)),
                    ExpectedType::Required => Err(Error::unexpected_type("Any", "assign", name)),
                    ExpectedType::None => {
                        let expected_type = typ.as_ref()
                            .map_or(Ok(ExpectedType::Required), |t| scope.get_type(t).map(ExpectedType::Type))?;
                        let (typ, expression) = self.expression(rhs, expected_type, scope)?.unwrap();
                        scope.add_local_value(name.value(), (typ, expression));
                        Ok(None)
                    }
                }
            },
        }
    }

    fn validate_expected_type<'a>(expected_type: ExpectedType, typ: VoidableType, token: &Token<'a>) -> Result<'a, ()> {
        match expected_type {
            ExpectedType::Type(expected_type) => {
                if let VoidableType::Type(typ) = typ {
                    if typ != expected_type {
                        Err(Error::unexpected_type(expected_type.to_str(), token.value(), token))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::unexpected_type(expected_type.to_str(), token.value(), token))
                }
            },
            ExpectedType::Required => {
                if typ.is_void() {
                    Err(Error::unexpected_type("Any", token.value(), token))
                } else { Ok(()) }
            },
            ExpectedType::None => Ok(()),
        }
    }

    // (block, has_return, last_statement)
    fn block<'a, 'module>(
        &self,
        node: &BlockNode<'a>,
        expected_type: ExpectedType,
        return_type: VoidableType,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, (bool, Option<(Type, BasicValueEnum<'ctx>)>)> {
        let mut scope = Scope::child(ScopeValues::empty(), scope);

        let has_return = node.ret.as_ref().is_some();

        let last_statement_ret = if let Some(
            (last_statement, statements)
        ) = node.statements.split_last() {
            for statement in statements {
                self.statement(statement, ExpectedType::None, &mut scope)?;
            }
            self.statement(last_statement, if has_return { ExpectedType::None } else { expected_type }, &mut scope)?
        } else {
            if !expected_type.is_none() {
                return Err(Error::unexpected_type(expected_type.to_str(), "Void", &node.close))
            }
            None 
        };

        if let Some(ret) = node.ret.as_ref() {
            if let Some(expression) = &ret.expression {
                if let VoidableType::Type(return_type) = return_type {
                    let (_, value) = self.expression(expression, ExpectedType::Type(return_type), &mut scope)?.unwrap();
                    self.builder.build_return(Some(&value));
                } else {
                    return Err(Error::unexpected_type("Void", ret.token.value(), &ret.token));
                }
            } else {
                if return_type.is_void() {
                    self.builder.build_return(None);
                } else {
                    return Err(Error::unexpected_type(return_type.to_str(), ret.token.value(), &ret.token));
                }
            }
        }

        Ok((has_return, if has_return { None } else { last_statement_ret }))
    }

    fn expression<'a, 'module>(
        &self,
        node: &ExpressionNode<'a>,
        expected_type: ExpectedType,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, Option<(Type, BasicValueEnum<'ctx>)>> {
        match node {
            ExpressionNode::Bool(value, token) => {
                LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Bool), token)?;
                Ok(Some((Type::Bool, self.context.bool_type().const_int(if *value { 1 } else { 0 }, false).into())))
            },
            ExpressionNode::Int(token) => {
                if let ExpectedType::Type(expected_type) = expected_type {
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
                let (typ, value) = scope.get_local_value(token)?;
                LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(*typ), token)?;
                Ok(Some((*typ, value.clone())))
            },
            ExpressionNode::FunctionCall { token, arguments } => {
                let function = scope.get_function(token)?;
                LLVMGenerator::validate_expected_type(expected_type, function.return_type, token)?;

                if function.arguments.len() != arguments.len() {
                    return Err(Error::unexpected_arguments_length(function.arguments.len(), arguments.len(), token))
                }

                let arguments = arguments.iter().enumerate().map(|(i, argument)| {
                    let (name, expected_type) = function.arguments[i];
                    if name != argument.label.value() {
                        return Err(Error::unexpected_label(name, &argument.label))
                    }
                    let (_, v) = self.expression(&argument.value, ExpectedType::Type(expected_type), scope)?.unwrap();
                    Ok(v.into())
                }).collect::<Result<Vec<BasicMetadataValueEnum>>>()?;

                Ok(function.build_call(&self.builder, arguments.as_slice()))
            },
            ExpressionNode::UnaryExpr { child, operator, token } => {
                match operator {
                    UnaryOperator::Minus => {
                        let (typ, child) = self.expression(child, expected_type, scope)?
                            .ok_or(Error::unexpected_type("Int, Double", "Void", token))?;
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
                let (_, rhs_value) = self.expression(rhs, ExpectedType::Type(lhs_type), scope)?.unwrap();
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
                
                let (lhs_type, lhs_value) = self.expression(lhs, ExpectedType::None, scope)?
                    .ok_or(Error::unexpected_type("Comparable", "Void", token))?;
                let (_, rhs_value) = self.expression(rhs, ExpectedType::Type(lhs_type), scope)?.unwrap();
                
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
            ExpressionNode::IfBranch { token, if_branches, else_branch } => {
                let current_function = scope.current_function.unwrap();
                let cont_block = self.context.append_basic_block(current_function.val, "cont");

                let mut expected_type = expected_type;
                let mut phi_incoming: Vec<(BasicValueEnum, BasicBlock<'ctx>)> = Vec::new();
                let mut all_return = true;

                for (condition, block) in if_branches {
                    let (_, condition) = self.expression(condition, ExpectedType::Type(Type::Bool), scope)?.unwrap();
                    let then_block = self.context.append_basic_block(current_function.val, "then");
                    let else_block = self.context.append_basic_block(current_function.val, "else");
                    
                    self.builder.build_conditional_branch(condition.into_int_value(), then_block, else_block);
                    
                    self.builder.position_at_end(then_block);
                    let (has_return, value) = self.block(block, expected_type, current_function.return_type, scope)?;
                    if !has_return {
                        all_return = false;
                        self.builder.build_unconditional_branch(cont_block);
                        if let Some((typ, value)) = value {
                            phi_incoming.push((value, then_block));
                            if expected_type.is_required() { expected_type = ExpectedType::Type(typ); }
                        }
                    }

                    self.builder.position_at_end(else_block);
                }

                if let Some(block) = else_branch {
                    let (has_return, value) = self.block(block, expected_type, current_function.return_type, scope)?;
                    if !has_return {
                        all_return = false;
                        self.builder.build_unconditional_branch(cont_block);
                        if let Some((typ, value)) = value {
                            phi_incoming.push((value, self.builder.get_insert_block().unwrap()));
                            if expected_type.is_required() { expected_type = ExpectedType::Type(typ); }
                        }
                    }
                } else if expected_type.is_none() {
                    all_return = false;
                    self.builder.build_unconditional_branch(cont_block);
                } else {
                    return Err(Error::unexpected_type(expected_type.to_str(), "Void", token));
                }

                if all_return { return Err(Error::all_return(token)); }

                self.builder.position_at_end(cont_block);

                match expected_type {
                    ExpectedType::Type(typ) => {
                        let phi = self.builder.build_phi(typ.to_basic_type_enum(self.context), "if_tmp");
                        let phi_incoming: Vec<_> = phi_incoming.iter().map(|(v, b)| (v as &dyn BasicValue, *b)).collect();
                        phi.add_incoming(phi_incoming.as_slice());
                        Ok(Some((typ, phi.as_basic_value())))
                    },
                    ExpectedType::None => Ok(None),
                    ExpectedType::Required => panic!("unexpected"),
                }
            }
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
