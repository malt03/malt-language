use std::ops::Range;
use super::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) text: &'a str,
    pub(crate) range: Range<usize>,
}

impl<'a> Token<'a> {
    pub(crate) fn new(
        kind: TokenKind,
        text: &'a str,
        range: Range<usize>,
    ) -> Token {
        Token { kind, text, range }
    }

    pub(crate) fn value(&self) -> &'a str {
        &self.text[self.range.clone()]
    }
}
