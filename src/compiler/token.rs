use std::ops::Range;
use super::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Token<'a> {
  kind: TokenKind,
  value: &'a str,
  range: Range<usize>,
}

impl<'a> Token<'a> {
  pub(crate) fn new(
    kind: TokenKind,
    value: &'a str,
    range: Range<usize>,
  ) -> Token {
    Token { kind, value, range }
  }
}
