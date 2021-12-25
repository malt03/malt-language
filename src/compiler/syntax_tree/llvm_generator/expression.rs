use crate::compiler::{tokens::Token, syntax_tree::{binary_operator::{CompareOperator, BinaryOperator}, syntax_tree_node::{ExpressionNode, CallArgumentNode, BlockNode}, unary_operator::UnaryOperator}};

use super::{error::{Result, Error}, typ::{Type, VoidableType, ExpectedType}, scope::Scope, LLVMGenerator};

use inkwell::{values::{BasicMetadataValueEnum, BasicValueEnum, BasicValue}, basic_block::BasicBlock};

impl<'ctx> LLVMGenerator<'ctx> {
    pub(super) fn expression<'a, 'module>(
        &self,
        node: &ExpressionNode<'a>,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        match node {
            ExpressionNode::Bool(value, token) => self.bool(expected_type, value, token),
            ExpressionNode::Int(token) => self.int(expected_type, token),
            ExpressionNode::Double(token) => self.double(expected_type, token),
            ExpressionNode::Identifier(token) => self.identifier(expected_type, scope, token),
            ExpressionNode::FunctionCall { token, arguments, name_with_arguments } => self.function_call(expected_type, scope, token, arguments, name_with_arguments),
            ExpressionNode::UnaryExpr { child, operator, token } => self.unary_expr(expected_type, scope, token, child, operator),
            ExpressionNode::BinaryExpr { lhs, rhs, operator, token } => self.binary_expr(expected_type, scope, token, lhs, rhs, operator),
            ExpressionNode::CompareExpr { token, lhs, rhs, operator } => self.compare_expr(expected_type, scope, token, lhs, rhs, operator),
            ExpressionNode::IfBranch { token, if_branches, else_branch } => self.if_branch(expected_type, scope, token, if_branches, else_branch),
            ExpressionNode::StructProperty { instance, property } => todo!(),
            ExpressionNode::StructConstruction { type_, arguments } => todo!(),
        }
    }

    fn bool<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        value: &bool,
        token: &Token<'a>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Bool), token)?;
        Ok(Some((Type::Bool, self.context.bool_type().const_int(if *value { 1 } else { 0 }, false).into())))
    }

    fn int<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        token: &Token<'a>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
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
    }

    fn double<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        token: &Token<'a>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(Type::Double), token)?;
        Ok(Some((
            Type::Double,
            self.context.f64_type().const_float_from_string(token.value()).into()
        )))
    }

    fn identifier<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        let (type_, value) = scope.get_local_value(token)?;
        LLVMGenerator::validate_expected_type(expected_type, VoidableType::Type(*type_), token)?;
        Ok(Some((*type_, value.clone())))
    }

    fn function_call<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
        arguments: &Vec<CallArgumentNode<'a>>,
        name_with_arguments: &String,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        let function = scope.get_function(token, name_with_arguments)?;
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
    }

    fn unary_expr<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
        child: &Box<ExpressionNode<'a>>,
        operator: &UnaryOperator,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        match operator {
            UnaryOperator::Minus => {
                let (type_, child) = self.expression(child, expected_type, scope)?
                    .ok_or(Error::unexpected_type("Int, Double", "Void", token))?;
                match type_ {
                    Type::Int => Ok(Some((
                        Type::Int,
                        self.builder.build_int_neg(child.into_int_value(), "negtmp").into(),
                    ))),
                    Type::Double => Ok(Some((
                        Type::Double,
                        self.builder.build_float_neg(child.into_float_value(), "negtmp").into(),
                    ))),
                    _ => Err(Error::unexpected_type(type_.to_str(), token.value(), token))
                }
            },
        }
    }

    fn binary_expr<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
        lhs: &Box<ExpressionNode<'a>>,
        rhs: &Box<ExpressionNode<'a>>,
        operator: &BinaryOperator,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
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
    }

    fn compare_expr<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
        lhs: &Box<ExpressionNode<'a>>,
        rhs: &Box<ExpressionNode<'a>>,
        operator: &CompareOperator,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
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
    }

    fn if_branch_block<'a, 'module>(
        &self,
        expected_type: &mut ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        block: &BlockNode<'a>,
        cont_block: BasicBlock<'ctx>,
        all_return: &mut bool,
        phi_incoming: &mut Vec<(BasicValueEnum<'ctx>, BasicBlock<'ctx>)>,
    ) -> Result<'a, ()> {
        let (has_return, value) = self.block(block, *expected_type, scope)?;
        if !has_return {
            *all_return = false;
            self.builder.build_unconditional_branch(cont_block);
            if let Some((type_, value)) = value {
                phi_incoming.push((value, self.builder.get_insert_block().unwrap()));
                if expected_type.is_required() { *expected_type = ExpectedType::Type(type_); }
            }
        }

        Ok(())
    }

    fn if_branch<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        token: &Token<'a>,
        if_branches: &Vec<(Box<ExpressionNode<'a>>, Box<BlockNode<'a>>)>,
        else_branch: &Option<Box<BlockNode<'a>>>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
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
            self.if_branch_block(&mut expected_type, scope, block, cont_block, &mut all_return, &mut phi_incoming)?;
            self.builder.position_at_end(else_block);
        }

        if let Some(block) = else_branch {
            self.if_branch_block(&mut expected_type, scope, block, cont_block, &mut all_return, &mut phi_incoming)?;
        } else if expected_type.is_none() {
            all_return = false;
            self.builder.build_unconditional_branch(cont_block);
        } else {
            return Err(Error::unexpected_type(expected_type.to_str(), "Void", token));
        }

        if all_return { return Err(Error::all_return(token)); }

        self.builder.position_at_end(cont_block);

        match expected_type {
            ExpectedType::Type(type_) => {
                let phi = self.builder.build_phi(type_.to_basic_type_enum(self.context), "if_tmp");
                let phi_incoming: Vec<_> = phi_incoming.iter().map(|(v, b)| (v as &dyn BasicValue, *b)).collect();
                phi.add_incoming(phi_incoming.as_slice());
                Ok(Some((type_, phi.as_basic_value())))
            },
            ExpectedType::None => Ok(None),
            ExpectedType::Required => panic!("unexpected"),
        }
    }

    fn struct_construction<'a, 'module>(
        &self,
        expected_type: ExpectedType<'a, 'ctx>,
        scope: &Scope<'a, 'module, 'ctx>,
        type_: &Token<'a>,
        arguments: &Vec<CallArgumentNode<'a>>,
    ) -> Result<'a, Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)>> {
        todo!()
    }
}
