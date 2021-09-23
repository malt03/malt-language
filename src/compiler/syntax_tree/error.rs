use super::super::tokens::{PeekableTokens, TokenKind};

#[derive(Debug)]
pub(crate) enum Error<'a> {
  UnexpectedToken { expected_kinds: Vec<TokenKind>, cursor: usize, text: &'a str },
}

impl<'a> Error<'a> {
  pub(crate) fn unexpected_token<T: IntoIterator<Item = TokenKind>>(
    expected_kinds: T,
    tokens: &PeekableTokens<'a>,
  ) -> Error<'a> {
    Error::UnexpectedToken {
      expected_kinds: expected_kinds.into_iter().collect(),
      cursor: tokens.cursor(),
      text: tokens.text(),
    }
  }
}

impl<'a> std::fmt::Display for Error<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("data")
  }
}

impl<'a> std::error::Error for Error<'a> {}

pub(crate) type Result<'a, T> = std::result::Result<T, Error<'a>>;
