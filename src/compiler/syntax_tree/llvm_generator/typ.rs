use std::{collections::HashMap, iter::FromIterator};
use super::error::{Result, Error};
use inkwell::{types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum}, context::Context, module::Module, values::{FunctionValue, BasicValueEnum, BasicMetadataValueEnum}, builder::Builder};

use crate::compiler::{syntax_tree::FunctionNode, tokens::Token};

#[derive(Clone)]
pub(crate) struct Function<'a, 'ctx> {
    pub(crate) name: &'a str,
    pub(crate) return_type: VoidableType,
    pub(crate) arguments: Vec<(&'a str, Type)>,
    pub(crate) val: FunctionValue<'ctx>,
}

impl<'a, 'ctx> Function<'a, 'ctx> {
    pub(crate) fn new(type_map: &TypeMap<'a>, node: &FunctionNode<'a>, context: &'ctx Context, module: &Module<'ctx>) -> Result<'a, Function<'a, 'ctx>> {
        let name = node.name.value();
        let return_type = node.return_type.as_ref()
            .map_or(
                Ok(VoidableType::Void),
                |t| type_map.get(t).map(VoidableType::Type),
            )?;
        
        let arguments = node.arguments.iter().map(|arg| {
            Ok((arg.name.value(), type_map.get(&arg.typ)?))
        }).collect::<Result<Vec<(&'a str, Type)>>>()?;

        let param_types = arguments.iter().map(|(_, typ)| {
            typ.to_basic_type_enum(context).into()
        }).collect::<Vec<BasicMetadataTypeEnum>>();
        
        let ty = return_type.type_to_fn_type(context, param_types.as_slice());
        let val = module.add_function(name, ty, None);

        Ok(Function { name, return_type, arguments, val })
    }

    pub(crate) fn build_call(&self, builder: &Builder<'ctx>, arguments: &[BasicMetadataValueEnum<'ctx>]) -> Option<(Type, BasicValueEnum<'ctx>)> {
        let call = builder.build_call(self.val, arguments, "calltmp");
        if let VoidableType::Type(typ) = self.return_type {
            Some((typ, call.try_as_basic_value().left().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Int,
    Double,
    Bool,
}

pub(crate) struct TypeMap<'a> {
    map: HashMap<&'a str, Type>,
}

impl<'a> TypeMap<'a> {
    pub(crate) fn new() -> TypeMap<'a> {
        let map = HashMap::from_iter([
            ("Int", Type::Int),
            ("Double", Type::Double),
            ("Bool", Type::Bool),
        ]);
        TypeMap { map }
    }

    pub(crate) fn get(&self, token: &Token<'a>) -> Result<'a, Type> {
        self.map.get(token.value()).map(|t| *t).ok_or(Error::type_not_found(token))
    }
}

impl<'ctx> Type {
    pub(crate) fn to_basic_type_enum(&self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Int => context.i64_type().into(),
            Type::Double => context.f64_type().into(),
            Type::Bool => context.bool_type().into(),
        }
    }

    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            Type::Int => "Int",
            Type::Double => "Double",
            Type::Bool => "Bool",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum VoidableType {
    Void,
    Type(Type),
}

impl<'ctx> VoidableType {
    fn type_to_fn_type(&self, context: &'ctx Context, param_types: &[BasicMetadataTypeEnum<'ctx>]) -> FunctionType<'ctx> {
        match self {
            VoidableType::Type(typ) => typ.to_basic_type_enum(context).fn_type(param_types, false),
            VoidableType::Void => context.void_type().fn_type(param_types, false),
        }
    }
}
