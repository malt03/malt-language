use std::collections::HashMap;
use std::io;

use super::error::{Result, Error};
use super::super::{LocalValue, BinaryOperator, ModuleNode, ExpressionNode, FunctionNode, StatementNode, SyntaxTree, UnaryOperator, Node};

struct Scope<'a, 'b> {
    functions: &'b HashMap<&'a str, &'b Node<'a, FunctionNode<'a>>>,
    values: &'b HashMap<&'a str, &'b Node<'a, LocalValue<'a>>>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn write_wasm<W: io::Write>(&self, writer: &mut W) -> Result<'a, ()> {
        self.write_module(writer, &self.root)
    }

    fn write_module<W: io::Write>(
        &self,
        writer: &mut W,
        program: &ModuleNode<'a>,
    ) -> Result<'a, ()> {
        let scope = Scope {
            functions: &program.functions.iter().map(|(k, v)| (*k, v)).collect(),
            values: &HashMap::new(),
        };
        for (_, function) in &program.functions {
            self.write_function(writer, &scope, &function.entity)?
        }
        Ok(())
    }

    fn write_function<'b, W: io::Write>(
        &self,
        writer: &mut W,
        scope: &'b Scope<'a, 'b>,
        function: &'b FunctionNode<'a>,
    ) -> Result<'a, ()> {
        let mut values: HashMap<&'a str, &'b Node<'a, LocalValue<'a>>> = HashMap::new();

        writer.write_fmt(format_args!("(func ${} ", function.name))?;
        for argument in &function.arguments {
            values.insert(argument.entity.name, argument);
            writer.write_fmt(format_args!("(param ${} i32)", argument.entity.name))?;
        }
        if function.return_.is_some() {
            writer.write_all(b"(result i32)")?;
        }
        for (_, local_value) in &function.local_values {
            values.insert(local_value.entity.name, local_value);
            writer.write_fmt(format_args!("(local ${} i32)", local_value.entity.name))?;
        }
        writer.write_all(b"\n")?;
        
        let values: HashMap<&'a str, &'b Node<'a, LocalValue<'a>>> = scope.values.iter().map(|(k, v)| (*k, *v)).chain(values).collect();
        let scope = Scope { functions: scope.functions, values: &values };
        for statement in &function.statements {
            self.write_statement(writer, &scope, statement)?;
        }
        if let Some(return_) = &function.return_ {
            self.write_expression(writer, &scope, &return_.entity.expression)?;
        }
        writer.write_all(b")\n")?;
        Ok(())
    }

    fn write_statement<'b, W: io::Write>(
        &self,
        writer: &mut W,
        scope: &Scope<'a, 'b>,
        statement: &StatementNode<'a>,
    ) -> Result<'a, &'a str> {
        let type_ = match statement {
            StatementNode::Expression(expression) => self.write_expression(writer, scope, &expression)?,
            StatementNode::Assign(name, expression) => {
                writer.write_fmt(format_args!("(local.set ${}", name.entity))?;
                self.write_expression(writer, scope, &expression)?;
                writer.write_all(b")")?;
                "Void"
            },
        };
        writer.write_all(b"\n")?;
        Ok(type_)
    }

    fn write_expression<'b, W: io::Write>(
        &self,
        writer: &mut W,
        scope: &Scope<'a, 'b>,
        expression: &Node<'a, ExpressionNode<'a>>,
    ) -> Result<'a, &'a str> {
        match &expression.entity {
            ExpressionNode::Value(value) => {
                writer.write_fmt(format_args!("(i32.const {})", value))?;
            },
            ExpressionNode::Identifier(name) => {
                writer.write_fmt(format_args!("(local.get ${})", name))?;
            },
            ExpressionNode::FunctionCall { name, arguments } => {
                let function = scope.functions.get(name)
                    .ok_or(Error::function_not_found(name, self.text, &expression.token))?;
                let expected_arguments = &function.entity.arguments;
                writer.write_fmt(format_args!("(call ${} ", name))?;
                
                for (i, argument) in arguments.iter().enumerate() {
                    self.write_expression(writer, scope, &argument)?;
                }
                writer.write_all(b")")?;
            },
            ExpressionNode::UnaryExpr { child, operator } => {
                match operator {
                    UnaryOperator::Minus => {
                        writer.write_all(b"(i32.sub (i32.const 0)")?;
                        self.write_expression(writer, scope, &child)?;
                        writer.write_all(b")")?;
                    },
                }
            },
            ExpressionNode::BinaryExpr { lhs, rhs, operator } => {
                let instruction: &[u8] = match operator {
                    BinaryOperator::Plus => b"i32.add",
                    BinaryOperator::Minus => b"i32.sub",
                    BinaryOperator::Multiply => b"i32.mul",
                    BinaryOperator::Divide => b"i32.div_s",
                };
                writer.write_all(b"(")?;
                writer.write_all(instruction)?;
                self.write_expression(writer, scope, &lhs)?;
                self.write_expression(writer, scope, &rhs)?;
                writer.write_all(b")")?;
            },
        }
        Ok("Void")
    }
}

#[cfg(test)]
mod tests {
    use super::SyntaxTree;
    use super::super::super::super::PeekableTokens;
    
    #[test]
    fn it_works() {
        let mut buffer: Vec<u8> = Vec::new();
        let code = r#"fn main() {
    foo: I32 = 2 + 3 * ((5 - 1) + 1) / 3
    bar: I32 = 10 - 4
    return foo + bar
}
"#;

        let expect = r#"(func $main (result i32)(local $foo i32)(local $bar i32)
(local.set $foo(i32.add(i32.const 2)(i32.div_s(i32.mul(i32.const 3)(i32.add(i32.sub(i32.const 5)(i32.const 1))(i32.const 1)))(i32.const 3))))
(local.set $bar(i32.sub(i32.const 10)(i32.const 4)))
(i32.add(local.get $foo)(local.get $bar)))
"#;

        let tree = SyntaxTree::new(PeekableTokens::new(code)).unwrap();
        tree.write_wasm(&mut buffer).unwrap();
        let wasm = String::from_utf8(buffer).unwrap();

        assert_eq!(wasm, expect);
    }
}
