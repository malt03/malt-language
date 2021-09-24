use std::io;
use super::{SyntaxTree, SyntaxTreeNode, BinaryOperator, UnaryOperator};

impl<'a> SyntaxTree<'a> {
    pub(crate) fn write_wasm<W: io::Write>(&self,  writer: &mut W) -> io::Result<()> {
        self.write_wasm_node(&self.root, writer)
    }

    fn write_wasm_node<W: io::Write>(&self, node: &SyntaxTreeNode<'a>, writer: &mut W) -> io::Result<()> {
        match node {
            SyntaxTreeNode::Value(value) => {
                writer.write_all(format!("(i32.const {})", value).as_bytes())?;
            },
            SyntaxTreeNode::UnaryExpr { child, operator } => {
                match operator {
                    UnaryOperator::Minus => {
                        writer.write_all(b"(i32.sub (i32.const 0)")?;
                        self.write_wasm_node(child, writer)?;
                        writer.write_all(b")")?;
                    },
                }
            },
            SyntaxTreeNode::BinaryExpr { lhs, rhs, operator } => {
                let instruction: &[u8] = match operator {
                    BinaryOperator::Plus => b"i32.add",
                    BinaryOperator::Minus => b"i32.sub",
                    BinaryOperator::Times => b"i32.mul",
                    BinaryOperator::Divide => b"i32.div_s",
                };
                writer.write_all(b"(")?;
                writer.write_all(instruction)?;
                self.write_wasm_node(lhs, writer)?;
                self.write_wasm_node(rhs, writer)?;
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
        let tree = SyntaxTree::new(PeekableTokens::new("2 + 3 * (5 - -(1 + 4)) / 2")).unwrap();
        tree.write_wasm(&mut buffer).unwrap();
        let wasm = String::from_utf8(buffer).unwrap();

        assert_eq!(wasm, "(i32.add(i32.const 2)(i32.div_s(i32.mul(i32.const 3)(i32.sub(i32.const 5)(i32.sub (i32.const 0)(i32.add(i32.const 1)(i32.const 4)))))(i32.const 2)))");
    }
}
