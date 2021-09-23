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
    match self {
      Error::UnexpectedToken { expected_kinds, cursor, text } => {

        let lines = text.split('\n');
        let mut read_len: usize = 0;
        for (line_number, line) in lines.enumerate() {
          let len = line.len();
          if read_len + len < *cursor {
            read_len += len;
            continue;
          }
          let line_cursor = *cursor - read_len;

          f.write_fmt(format_args!("Unexpected token found. line: {}\n", line_number + 1))?;
          f.write_fmt(format_args!("Expected: {:?}\n\n", expected_kinds))?;
          f.write_fmt(format_args!("{}\n", line))?;
          f.write_fmt(format_args!("{}^\n", " ".repeat(line_cursor - 1)))?;
        }

        return Ok(())
      },
    }
  }
}

impl<'a> std::error::Error for Error<'a> {}

pub(crate) type Result<'a, T> = std::result::Result<T, Error<'a>>;
