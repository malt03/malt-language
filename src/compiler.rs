mod token;
mod token_kind;
mod token_kind_converter;

use std::io;

pub fn compile<Lines: Iterator<Item = io::Result<String>>>(lines: Lines) {
  for line in lines {
    match line {
      Ok(line) => println!("{}", line),
      Err(err) => println!("{:?}", err),
    }
  }
}
