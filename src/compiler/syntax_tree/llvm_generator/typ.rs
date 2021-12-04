use std::str::FromStr;

use inkwell::{types::BasicTypeEnum, context::Context};

pub(crate) enum Type {
    Int,
    Double,
    Bool,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Int" => Ok(Type::Int),
            "Double" => Ok(Type::Double),
            "Bool" => Ok(Type::Bool),
            _ => Err(()),
        }
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
}
