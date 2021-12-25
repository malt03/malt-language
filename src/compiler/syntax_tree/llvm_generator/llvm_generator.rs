use std::collections::HashMap;

use crate::compiler::syntax_tree::{syntax_tree_node::{ModuleNode, FunctionNode, StatementNode, BlockNode}, binary_operator::CompareOperator};

use super::{error::{Result, Error}, typ::{Function, Type, VoidableType, ExpectedType}, scope::{ScopeValues, Scope}};

use inkwell::{builder::Builder, context::Context, values::BasicValueEnum, module::Module, IntPredicate, FloatPredicate};
use super::super::super::tokens::Token;

pub(crate) struct LLVMGenerator<'ctx> {
    pub(super) context: &'ctx Context,
    pub(super) builder: Builder<'ctx>,
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
            let function = Function::new(&scope, function, self.context, &module)?;
            Ok((function.name_with_arguments.clone(), function))
        }).collect::<Result<HashMap<_, _>>>()?;

        let scope = Scope::module(ScopeValues::new(HashMap::new(), functions));
        for function in &node.functions {
            self.function(function, &scope)?;
        }
        
        Ok(module)
    }

    fn function<'a, 'module>(&self, node: &FunctionNode<'a>, scope: &Scope<'a, 'module, 'ctx>) -> Result<'a, ()> {
        let function = scope.get_function(&node.name, &node.name_with_arguments)?;

        let local_values: HashMap<&'a str, (Type, BasicValueEnum<'ctx>)> = function.val.get_param_iter().enumerate().map(|(i, arg)| {
            let (name, type_) = function.arguments[i];
            (name, (type_, arg.into()))
        }).collect();

        let scope = Scope::function(
            function,
            ScopeValues::new(local_values, HashMap::new()),
            scope,
        );

        let block = self.context.append_basic_block(scope.current_function.unwrap().val, "block");
        self.builder.position_at_end(block);
        let (has_return, _) = self.block(&node.block, ExpectedType::None, &scope)?;

        if !has_return && function.return_type.is_type() {
            Err(Error::unexpected_type(function.return_type.to_str(), "Void", &node.block.close))
        } else {
            Ok(())
        }
    }

    fn statement<'a, 'module>(
        &self,
        node: &StatementNode<'a>,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &mut Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        match node {
            StatementNode::Expression(expression) => {
                self.expression(expression, expected_type, scope)
            }
            StatementNode::Assign { name, type_, rhs } => {
                match expected_type {
                    ExpectedType::Type(expected_type) => Err(Error::unexpected_type(expected_type.to_str(), "assign", name)),
                    ExpectedType::Required => Err(Error::unexpected_type("Any", "assign", name)),
                    ExpectedType::None => {
                        let expected_type = type_.as_ref()
                            .map_or(Ok(ExpectedType::Required), |t| scope.get_type(t).map(ExpectedType::Type))?;
                        let (type_, expression) = self.expression(rhs, expected_type, scope)?.unwrap();
                        scope.add_local_value(name.value(), (type_, expression));
                        Ok(None)
                    }
                }
            },
        }
    }

    pub(super) fn validate_expected_type<'a>(expected_type: ExpectedType<'a, 'ctx>, type_: VoidableType<'a, 'ctx>, token: &Token<'a>) -> Result<'a, ()> {
        match expected_type {
            ExpectedType::Type(expected_type) => {
                if let VoidableType::Type(type_) = type_ {
                    if type_ != expected_type {
                        Err(Error::unexpected_type(expected_type.to_str(), token.value(), token))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::unexpected_type(expected_type.to_str(), token.value(), token))
                }
            },
            ExpectedType::Required => {
                if type_.is_void() {
                    Err(Error::unexpected_type("Any", token.value(), token))
                } else { Ok(()) }
            },
            ExpectedType::None => Ok(()),
        }
    }

    // (block, has_return, last_statement)
    pub(super) fn block<'a, 'module>(
        &self,
        node: &BlockNode<'a>,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, (bool, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>)> {
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

        let return_type = scope.current_function.unwrap().return_type;
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
