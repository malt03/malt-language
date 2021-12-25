use std::collections::HashMap;

use super::{error::{Result, Error}, typ::{TypeMap, Function, Type}};

use inkwell::values::BasicValueEnum;
use super::super::super::tokens::Token;

pub(crate) struct ScopeValues<'a, 'ctx> {
    local_values: HashMap<&'a str, (Type, BasicValueEnum<'ctx>)>,
    functions: HashMap<String, Function<'a, 'ctx>>,
    type_map: TypeMap<'a>,
}

impl<'a, 'ctx> ScopeValues<'a, 'ctx> {
    pub(crate) fn empty() -> ScopeValues<'a, 'ctx> {
        ScopeValues::new(HashMap::new(), HashMap::new())
    }

    pub(crate) fn new(
        local_values: HashMap<&'a str, (Type, BasicValueEnum<'ctx>)>,
        functions: HashMap<String, Function<'a, 'ctx>>,
    ) -> ScopeValues<'a, 'ctx> {
        ScopeValues { local_values, functions, type_map: TypeMap::new() }
    }
}

pub(crate) struct Scope<'a, 'module, 'ctx> {
    pub(crate) current_function: Option<&'module Function<'a, 'ctx>>,
    values: ScopeValues<'a, 'ctx>,
    above: Option<&'module Scope<'a, 'module, 'ctx>>,
}

impl<'a, 'module, 'ctx> Scope<'a, 'module, 'ctx> {
    pub(crate) fn module(
        values: ScopeValues<'a, 'ctx>,
    ) -> Scope<'a, 'module, 'ctx> {
        Scope { current_function: None, values, above: None }
    }

    pub(crate) fn function(
        current_function: &'module Function<'a, 'ctx>,
        values: ScopeValues<'a, 'ctx>,
        above: &'module Scope<'a, 'module, 'ctx>,
    ) -> Scope<'a, 'module, 'ctx> {
        Scope { current_function: Some(current_function), values, above: Some(above) }
    }

    pub(crate) fn child(
        values: ScopeValues<'a, 'ctx>,
        above: &'module Scope<'a, 'module, 'ctx>,
    ) -> Scope<'a, 'module, 'ctx> {
        Scope { current_function: above.current_function, values, above: Some(above) }
    }

    fn search<'s, T, F: Fn(&'s ScopeValues<'a, 'ctx>) -> Option<T>>(&'s self, f: F) -> Option<T> {
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

    pub(crate) fn add_local_value(&mut self, name: &'a str, value: (Type, BasicValueEnum<'ctx>)) {
        self.values.local_values.insert(name, value);
    }

    pub(crate) fn get_function<'s>(&'s self, token: &Token<'a>, name_with_arguments: &String) -> Result<'a, &'s Function<'a, 'ctx>> {
        self.search(|scope| scope.functions.get(name_with_arguments)).ok_or(Error::function_not_found(token))
    }

    pub(crate) fn get_local_value(&self, token: &Token<'a>) -> Result<'a, &(Type, BasicValueEnum<'ctx>)> {
        self.search(|scope| scope.local_values.get(token.value())).ok_or(Error::value_not_found(token))
    }

    pub(crate) fn get_type(&self, token: &Token<'a>) -> Result<'a, Type> {
        self.search(|scope| scope.type_map.get(token)).ok_or(Error::type_not_found(token))
    }
}
