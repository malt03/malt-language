use std::ops::Range;
use super::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) value: &'a str,
    pub(crate) range: Range<usize>,
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
