use std::collections::HashMap;
use std::fs::File;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Range;
use std::io::{self, BufRead};
use super::token_kind::TokenKind;
use super::token_kind_converter::TokenKindConverter;

pub(crate) struct Token {
  kind: TokenKind,
  line: usize,
  range: Range<usize>,
}

pub(crate) struct Tokens<'a, Lines: Iterator<Item = io::Result<String>>> {
  converter: TokenKindConverter<'a>,
  lines: Lines,
  line: &'a str,
  cursor: usize,
}

impl<'a, Lines: Iterator<Item = io::Result<String>>> Iterator for Tokens<'a, Lines> {
  type Item = Token;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.converter.convert(self.line, &mut self.cursor);
    todo!()
    // loop {
    //   return match self.peek()? {
    //     space if space.is_whitespace() => {
    //       self.advance();
    //       continue;
    //     },
    //     '(' => self.chomp(TokenKind::OpenParen),
    //     ')' => self.chomp(TokenKind::CloseParen),
    //     '+' => self.chomp(TokenKind::Plus),
    //     '-' => self.chomp(TokenKind::Minus),
    //     '*' => self.chomp(TokenKind::Times),
    //     '/' => self.chomp(TokenKind::Divide),
    //     '_' | 'a'..='z' | 'A'..='Z' => {
    //       Some(Ok(self.chomp_identifier()))
    //     },
    //     '0'..='9' => Some(Ok(self.chomp_number())),
    //     other => Some(Err(ParseError::InvalidCharacter {
    //       character: other,
    //       index: self.cursor,
    //     })),
    //   };
    // }
  }
}
