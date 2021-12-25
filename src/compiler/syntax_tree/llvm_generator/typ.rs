use std::{collections::HashMap, iter::FromIterator};
use super::{error::Result, scope::Scope};
use inkwell::{types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum, StructType}, context::Context, module::Module, values::{FunctionValue, BasicValueEnum, BasicMetadataValueEnum}, builder::Builder};

use crate::compiler::{tokens::Token, syntax_tree::syntax_tree_node::FunctionNode};

#[derive(Clone)]
pub(crate) struct Function<'a, 'ctx> {
    pub(crate) name_with_arguments: String,
    pub(crate) return_type: VoidableType<'a, 'ctx>,
    pub(crate) arguments: Vec<(&'a str, Type<'a, 'ctx>)>,
    pub(crate) val: FunctionValue<'ctx>,
}

impl<'a, 'ctx> Function<'a, 'ctx> {
    pub(crate) fn new<'module>(scope: &Scope<'a, 'module, 'ctx>, node: &FunctionNode<'a>, context: &'ctx Context, module: &Module<'ctx>) -> Result<'a, Function<'a, 'ctx>> {
        let name_with_arguments = node.name_with_arguments.clone();

        let return_type = node.return_type.as_ref()
            .map_or(
                Ok(VoidableType::Void),
                |t| scope.get_type(t).map(VoidableType::Type),
            )?;
        
        let arguments = node.arguments.iter().map(|arg| {
            Ok((arg.name.value(), scope.get_type(&arg.type_)?))
        }).collect::<Result<Vec<(&'a str, Type)>>>()?;

        let param_types = arguments.iter().map(|(_, type_)| {
            type_.to_basic_type_enum(context).into()
        }).collect::<Vec<BasicMetadataTypeEnum>>();
        
        let ty = return_type.type_to_fn_type(context, param_types.as_slice());
        let val = module.add_function(&name_with_arguments, ty, None);

        Ok(Function { name_with_arguments, return_type, arguments, val })
    }

    pub(crate) fn build_call(&self, builder: &Builder<'ctx>, arguments: &[BasicMetadataValueEnum<'ctx>]) -> Option<(Type<'a, 'ctx>, BasicValueEnum<'ctx>)> {
        let call = builder.build_call(self.val, arguments, "calltmp");
        if let VoidableType::Type(type_) = self.return_type {
            Some((type_, call.try_as_basic_value().left().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Type<'a, 'ctx> {
    Int,
    Double,
    Bool,
    Struct(&'a str, StructType<'ctx>),
}

pub(crate) struct TypeMap<'a, 'ctx> {
    map: HashMap<&'a str, Type<'a, 'ctx>>,
}

impl<'a, 'ctx> TypeMap<'a, 'ctx> {
    pub(crate) fn new() -> TypeMap<'a, 'ctx> {
        let map = HashMap::from_iter([
            ("Int", Type::Int),
            ("Double", Type::Double),
            ("Bool", Type::Bool),
        ]);
        TypeMap { map }
    }

    pub(crate) fn get(&self, token: &Token<'a>) -> Option<Type<'a, 'ctx>> {
        self.map.get(token.value()).map(|t| *t)
    }
}

impl<'a, 'ctx> Type<'a, 'ctx> {
    pub(crate) fn to_basic_type_enum(&self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Int => context.i64_type().into(),
            Type::Double => context.f64_type().into(),
            Type::Bool => context.bool_type().into(),
            Type::Struct(_, t) => t.as_basic_type_enum(),
        }
    }

    pub(crate) fn to_str(&self) -> &'a str {
        match self {
            Type::Int => "Int",
            Type::Double => "Double",
            Type::Bool => "Bool",
            Type::Struct(name, _) => name,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum VoidableType<'a, 'ctx> {
    Void,
    Type(Type<'a, 'ctx>),
}

impl<'a, 'ctx> VoidableType<'a, 'ctx> {
    fn type_to_fn_type(&self, context: &'ctx Context, param_types: &[BasicMetadataTypeEnum<'ctx>]) -> FunctionType<'ctx> {
        match self {
            VoidableType::Type(type_) => type_.to_basic_type_enum(context).fn_type(param_types, false),
            VoidableType::Void => context.void_type().fn_type(param_types, false),
        }
    }

    pub(crate) fn is_void(&self) -> bool {
        if let VoidableType::Type(_) = self { false } else { true }
    }

    pub(crate) fn is_type(&self) -> bool {
        match self {
            VoidableType::Type(_) => true,
            _ => false,
        }
    }

    pub(crate) fn to_str(&self) -> &'a str {
        match self {
            VoidableType::Type(type_) => type_.to_str(),
            VoidableType::Void => "Void",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ExpectedType<'a, 'ctx> {
    Required,
    None,
    Type(Type<'a, 'ctx>),
}

impl<'a, 'ctx> ExpectedType<'a, 'ctx> {
    pub(crate) fn to_str(&self) -> &'a str {
        match self {
            ExpectedType::Type(type_) => type_.to_str(),
            ExpectedType::None => "None",
            ExpectedType::Required => "Any",
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        match self {
            ExpectedType::None => true,
            _ => false,
        }
    }

    pub(crate) fn is_required(&self) -> bool {
        match self {
            ExpectedType::Required => true,
            _ => false,
        }
    }
}
