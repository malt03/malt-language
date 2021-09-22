mod token_kind;
mod token_converter;
mod token;
mod tokens;

pub(crate) use token_kind::TokenKind;
pub(crate) use token_converter::TokenConverter;
pub(crate) use token::Token;
pub(crate) use tokens::Tokens;

use std::io;

pub fn compile<Lines: Iterator<Item = io::Result<String>>>(lines: Lines) {
  for line in lines {
    match line {
      Ok(line) => println!("{}", line),
      Err(err) => println!("{:?}", err),
    }
  }
}
