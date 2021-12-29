use std::collections::HashMap;

use super::{error::{Result, Error}, typ::{TypeMap, Function, Type, Struct}};

use inkwell::values::BasicValueEnum;
use super::super::super::tokens::Token;

#[derive(Debug)]
pub(crate) struct ScopeValues<'a, 'module, 'ctx> {
    local_values: HashMap<&'a str, (Type<'a, 'module, 'ctx>, BasicValueEnum<'ctx>)>,
    functions: HashMap<String, Function<'a, 'module, 'ctx>>,
    type_map: TypeMap<'a, 'module, 'ctx>,
}

impl<'a, 'module, 'ctx> ScopeValues<'a, 'module, 'ctx> {
    pub(crate) fn empty() -> ScopeValues<'a, 'module, 'ctx> {
        ScopeValues::new(HashMap::new(), HashMap::new())
    }

    pub(crate) fn new(
        local_values: HashMap<&'a str, (Type<'a, 'module, 'ctx>, BasicValueEnum<'ctx>)>,
        functions: HashMap<String, Function<'a, 'module, 'ctx>>,
    ) -> ScopeValues<'a, 'module, 'ctx> {
        ScopeValues { local_values, functions, type_map: TypeMap::new() }
    }
}

#[derive(Debug)]
pub(crate) struct Scope<'a, 'module, 'ctx, 's> {
    pub(crate) current_function: Option<&'module Function<'a, 'module, 'ctx>>,
    values: ScopeValues<'a, 'module, 'ctx>,
    above: Option<&'s Scope<'a, 'module, 'ctx, 's>>,
}

impl<'a, 'module, 'ctx, 's> Scope<'a, 'module, 'ctx, 's> {
    pub(crate) fn module(
        values: ScopeValues<'a, 'module, 'ctx>,
    ) -> Scope<'a, 'module, 'ctx, 's> {
        Scope { current_function: None, values, above: None }
    }

    pub(crate) fn function(
        current_function: &'module Function<'a, 'module, 'ctx>,
        values: ScopeValues<'a, 'module, 'ctx>,
        above: &'s Scope<'a, 'module, 'ctx, 's>,
    ) -> Scope<'a, 'module, 'ctx, 's> {
        Scope { current_function: Some(current_function), values, above: Some(above) }
    }

    pub(crate) fn child(
        values: ScopeValues<'a, 'module, 'ctx>,
        above: &'s Scope<'a, 'module, 'ctx, 's>,
    ) -> Scope<'a, 'module, 'ctx, 's> {
        Scope { current_function: above.current_function, values, above: Some(above) }
    }

    fn search<T, F: Fn(&'s ScopeValues<'a, 'module, 'ctx>) -> Option<T>>(&'s self, f: F) -> Option<T> {
        if let Some(result) = f(&self.values) {
            Some(result)
        } else {
            if let Some(above) = self.above {
                above.search(f)
            } else {
                None
            }
        }
    }

    pub(crate) fn set_functions(&mut self, functions: HashMap<String, Function<'a, 'module, 'ctx>>) {
        self.values.functions = functions;
    }

    pub(crate) fn add_local_value(&mut self, name: &'a str, value: (Type<'a, 'module, 'ctx>, BasicValueEnum<'ctx>)) {
        self.values.local_values.insert(name, value);
    }

    pub(crate) fn add_struct(&mut self, struct_: &'module Struct<'a, 'module, 'ctx>) {
        self.values.type_map.add_struct(struct_);
    }

    pub(crate) fn get_function(&'s self, token: &Token<'a>, name_with_arguments: &String) -> Result<'a, &'s Function<'a, 'module, 'ctx>> {
        self.search(|scope| scope.functions.get(name_with_arguments)).ok_or(Error::function_not_found(token))
    }

    pub(crate) fn get_local_value(&self, token: &Token<'a>) -> Result<'a, &(Type<'a, 'module, 'ctx>, BasicValueEnum<'ctx>)> {
        self.search(|scope| scope.local_values.get(token.value())).ok_or(Error::value_not_found(token))
    }

    pub(crate) fn get_type(&self, token: &Token<'a>) -> Result<'a, Type<'a, 'module, 'ctx>> {
        self.search(|scope| scope.type_map.get(token)).ok_or(Error::type_not_found(token))
    }
}
