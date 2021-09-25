mod tokens;
mod syntax_tree;

use std::{fs, io};

use tokens::PeekableTokens;
use syntax_tree::SyntaxTree;

#[derive(Debug)]
pub enum Error<'a> {
    IO(io::Error),
    SyntaxTree(syntax_tree::Error<'a>),
    GenerateWasm(syntax_tree::generate_wasm::Error<'a>),
}
impl<'a> From<io::Error> for Error<'a> {
    fn from(err: io::Error) -> Self { Error::IO(err) }
}
impl<'a> From<syntax_tree::Error<'a>> for Error<'a> {
    fn from(err: syntax_tree::Error<'a>) -> Self { Error::SyntaxTree(err) }
}
impl<'a> From<syntax_tree::generate_wasm::Error<'a>> for Error<'a> {
    fn from(err: syntax_tree::generate_wasm::Error<'a>) -> Self { Error::GenerateWasm(err) }
}
impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => err.fmt(f),
            Error::SyntaxTree(err) => err.fmt(f),
            Error::GenerateWasm(err) => err.fmt(f),
        }
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

pub fn compile<'a, W: io::Write>(text: &'a str, mut writer: W) -> Result<'a, ()> {
    writer.write_all(br#"(module
(import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
"#)?;

    let tokens = PeekableTokens::new(&text);
    let syntax_tree = SyntaxTree::new(tokens)?;
    syntax_tree.write_wasm(&mut writer)?;

    let malloc = fs::read("./lib/malloc.wat")?;
    writer.write_all(malloc.as_slice())?;
    writer.write_all(br#"(func $_start
call $main
call $_exit)
(memory 2)
(export "memory" (memory 0))
(export "_start" (func $_start)))"#)?;
    writer.flush()?;

    Ok(())
}
