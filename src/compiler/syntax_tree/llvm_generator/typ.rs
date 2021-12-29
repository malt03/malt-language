use std::{collections::HashMap, iter::FromIterator};
use super::{error::Result, scope::Scope, Error};
use inkwell::{types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum, StructType}, context::Context, module::Module, values::{FunctionValue, BasicValueEnum, BasicMetadataValueEnum}, builder::Builder};

use crate::compiler::{tokens::Token, syntax_tree::syntax_tree_node::{FunctionNode, StructNode}};

#[derive(Clone, Debug)]
pub(crate) struct Function<'a, 'module, 'ctx> {
    pub(crate) name_with_arguments: String,
    pub(crate) return_type: VoidableType<'a, 'module, 'ctx>,
    pub(crate) arguments: Vec<(&'a str, Type<'a, 'module, 'ctx>)>,
    pub(crate) val: FunctionValue<'ctx>,
}

impl<'a, 'module, 'ctx> Function<'a, 'module, 'ctx> {
    pub(crate) fn new<'s>(
        scope: &Scope<'a, 'module, 'ctx, 's>,
        node: &FunctionNode<'a>,
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> Result<'a, Function<'a, 'module, 'ctx>> {
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

    pub(crate) fn build_call(&self, builder: &Builder<'ctx>, arguments: &[BasicMetadataValueEnum<'ctx>]) -> Option<(Type<'a, 'module, 'ctx>, BasicValueEnum<'ctx>)> {
        let call = builder.build_call(self.val, arguments, "calltmp");
        if let VoidableType::Type(type_) = self.return_type {
            Some((type_, call.try_as_basic_value().left().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Struct<'a, 'module, 'ctx> {
    pub(crate) name: &'a str,
    pub(crate) properties: Vec<(&'a str, Type<'a, 'module, 'ctx>)>,
    pub(crate) properties_map: HashMap<&'a str, (u32, Type<'a, 'module, 'ctx>)>,
    pub(crate) type_: StructType<'ctx>,
}

impl<'a, 'module, 'ctx> Struct<'a, 'module, 'ctx> {
    pub(crate) fn new<'s>(
        scope: &Scope<'a, 'module, 'ctx, 's>,
        node: &StructNode<'a>,
        context: &'ctx Context,
    ) -> Result<'a, Struct<'a, 'module, 'ctx>> {
        let properties = node.properties.iter().map(|property| {
            Ok((property.name.value(), scope.get_type(&property.type_)?))
        }).collect::<Result<Vec<(&'a str, Type)>>>()?;

        let properties_map: HashMap<_, _> = properties.iter().enumerate().map(|(i, (name, type_))| {
            (*name, (i as u32, *type_))
        }).collect();

        let property_types: Vec<_> = properties.iter().map(|(_, type_)| type_.to_basic_type_enum(context)).collect();

        let type_ = context.struct_type(property_types.as_slice(), true);
        
        Ok(Struct { name: node.name.value(), properties, properties_map, type_ })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Type<'a, 'module, 'ctx> {
    Int,
    Double,
    Bool,
    Struct(&'module Struct<'a, 'module, 'ctx>),
}

#[derive(Debug)]
pub(crate) struct TypeMap<'a, 'module, 'ctx> {
    map: HashMap<&'a str, Type<'a, 'module, 'ctx>>,
}

impl<'a, 'module, 'ctx> TypeMap<'a, 'module, 'ctx> {
    pub(crate) fn new() -> TypeMap<'a, 'module, 'ctx> {
        let map = HashMap::from_iter([
            ("Int", Type::Int),
            ("Double", Type::Double),
            ("Bool", Type::Bool),
        ]);
        TypeMap { map }
    }

    pub(crate) fn add_struct(&mut self, struct_: &'module Struct<'a, 'module, 'ctx>) {
        self.map.insert(struct_.name, Type::Struct(struct_));
    }

    pub(crate) fn get(&self, token: &Token<'a>) -> Option<Type<'a, 'module, 'ctx>> {
        self.map.get(token.value()).map(|t| *t)
    }
}


impl<'a, 'module, 'ctx> Type<'a, 'module, 'ctx> {
    pub(crate) fn to_basic_type_enum(&self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Int => context.i64_type().into(),
            Type::Double => context.f64_type().into(),
            Type::Bool => context.bool_type().into(),
            Type::Struct(s) => s.type_.as_basic_type_enum(),
        }
    }

    pub(crate) fn as_struct(&self, token: &Token<'a>) -> Result<'a, &'module Struct<'a, 'module, 'ctx>> {
        if let Type::Struct(s) = self {
            Ok(s)
        } else {
            Err(Error::unexpected_type("struct", self.to_str(), token))
        }
    }

    pub(crate) fn to_str(&self) -> &'a str {
        match self {
            Type::Int => "Int",
            Type::Double => "Double",
            Type::Bool => "Bool",
            Type::Struct(s) => s.name,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum VoidableType<'a, 'module, 'ctx> {
    Void,
    Type(Type<'a, 'module, 'ctx>),
}

impl<'a, 'module, 'ctx> VoidableType<'a, 'module, 'ctx> {
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
pub(crate) enum ExpectedType<'a, 'module, 'ctx> {
    Required,
    None,
    Type(Type<'a, 'module, 'ctx>),
}

impl<'a, 'module, 'ctx> ExpectedType<'a, 'module, 'ctx> {
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
