use std::io;
use super::{SyntaxTree, BinaryOperator};

impl<'a> SyntaxTree<'a> {
  pub(crate) fn write_wasm<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
    match self {
      SyntaxTree::Value(value) => {
        writer.write_all(format!("        i32.const {}\n", value).as_bytes())?;
      },
      SyntaxTree::BinaryExpr { lhs, rhs, operator } => {
        lhs.write_wasm(writer)?;
        rhs.write_wasm(writer)?;
        match operator {
          BinaryOperator::Plus => writer.write_all(b"        i32.add\n")?,
          BinaryOperator::Minus => writer.write_all(b"        i32.sub\n")?,
          BinaryOperator::Times => writer.write_all(b"        i32.mul\n")?,
          BinaryOperator::Divide => writer.write_all(b"        i32.div_s\n")?,
        }
      },
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::SyntaxTree;
  use super::super::super::Tokens;

  #[test]
  fn it_works() {
    let mut buffer: Vec<u8> = Vec::new();
    let tree = SyntaxTree::new(&mut Tokens::new("2 + 3 * (5 - (1 + 4)) / 2").peekable());
    tree.write_wasm(&mut buffer).unwrap();
    let wasm = String::from_utf8(buffer).unwrap();

    assert_eq!(wasm, r#"        i32.const 2
        i32.const 3
        i32.const 5
        i32.const 1
        i32.const 4
        i32.add
        i32.sub
        i32.mul
        i32.const 2
        i32.div_s
        i32.add
"#);
  }
}
