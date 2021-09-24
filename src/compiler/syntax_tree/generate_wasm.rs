use std::io;
use super::{BinaryOperator, ExpressionNode, FunctionNode, StatementNode, SyntaxTree, UnaryOperator};

impl<'a> SyntaxTree<'a> {
    pub(crate) fn write_wasm<W: io::Write>(&self,  writer: &mut W) -> io::Result<()> {
        self.write_program(writer, &self.root)
    }

    fn write_program<W: io::Write>(&self, writer: &mut W, program: &FunctionNode<'a>) -> io::Result<()> {
        writer.write_all(b"(func $main ")?;
        if program.return_statement.is_some() {
            writer.write_all(b"(result i32)")?;
        }
        for local_value in &program.local_values {
            writer.write_fmt(format_args!("(local ${} i32)", local_value.name))?;
        }
        writer.write_all(b"\n")?;
        for statement in &program.statements {
            self.write_statement(writer, statement)?
        }
        if let Some(expression) = &program.return_statement {
            self.write_expression(writer, expression)?
        }
        writer.write_all(b")")?;
        Ok(())
    }

    fn write_statement<W: io::Write>(&self, writer: &mut W, statement: &StatementNode<'a>) -> io::Result<()> {
        match statement {
            StatementNode::Expression(expression) => self.write_expression(writer, expression)?,
            StatementNode::Assign(name, expression) => {
                writer.write_fmt(format_args!("(local.set ${}", name))?;
                self.write_expression(writer, expression)?;
                writer.write_all(b")")?;
            },
        }
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn write_expression<W: io::Write>(&self, writer: &mut W, expression: &ExpressionNode<'a>) -> io::Result<()> {
        match expression {
            ExpressionNode::Value(value) => {
                writer.write_fmt(format_args!("(i32.const {})", value))?;
            },
            ExpressionNode::Identifier(name) => {
                writer.write_fmt(format_args!("(local.get ${})", name))?;
            },
            ExpressionNode::UnaryExpr { child, operator } => {
                match operator {
                    UnaryOperator::Minus => {
                        writer.write_all(b"(i32.sub (i32.const 0)")?;
                        self.write_expression(writer, child)?;
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
                self.write_expression(writer, lhs)?;
                self.write_expression(writer, rhs)?;
                writer.write_all(b")")?;
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SyntaxTree;
    use super::super::super::PeekableTokens;
    
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
(i32.add(local.get $foo)(local.get $bar)))"#;

        let tree = SyntaxTree::new(PeekableTokens::new(code)).unwrap();
        tree.write_wasm(&mut buffer).unwrap();
        let wasm = String::from_utf8(buffer).unwrap();

        assert_eq!(wasm, expect);
    }
}
