mod tokens;
mod syntax_tree;

use std::io;

use tokens::PeekableTokens;
use syntax_tree::SyntaxTree;

// #[derive(Debug)]
// enum Error {
//   IO(io::Error),
//   SyntaxTree(syntax_tree::)
// }

pub fn compile<W: io::Write>(text: String, mut writer: W) -> io::Result<()> {
  writer.write_all(br#"(module
(import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
(func $_start
"#)?;

  let tokens = PeekableTokens::new(&text);
  let syntax_tree = SyntaxTree::new(tokens).unwrap();
  syntax_tree.write_wasm(&mut writer)?;

  writer.write_all(br#"
call $_exit)
(memory 0)
(export "memory" (memory 0))
(export "_start" (func $_start)))"#)?;
  writer.flush()?;

  Ok(())
}
