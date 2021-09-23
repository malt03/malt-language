mod tokens;
mod syntax_tree;

use std::io;

use tokens::Tokens;
use syntax_tree::SyntaxTree;

pub fn compile<W: io::Write>(text: String, mut writer: W) -> io::Result<()> {
  writer.write_all(br#"(module
(import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
(func $_start
"#)?;

  let tokens = Tokens::new(&text);
  let syntax_tree = SyntaxTree::new(&mut tokens.peekable());
  syntax_tree.write_wasm(&mut writer)?;

  writer.write_all(br#"
call $_exit)
(memory 0)
(export "memory" (memory 0))
(export "_start" (func $_start)))"#)?;
  writer.flush()?;

  Ok(())
}
