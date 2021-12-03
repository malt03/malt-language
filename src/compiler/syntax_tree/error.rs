use super::super::tokens::{self, PeekableTokens, TokenKind, Token};

#[derive(Debug)]
pub enum Error<'a> {
    UnexpectedToken { expected_kinds: Vec<TokenKind>, kind: TokenKind, cursor: usize, text: &'a str },
    Tokens(tokens::Error<'a>),
}

impl<'a> From<tokens::Error<'a>> for Error<'a> {
    fn from(err: tokens::Error<'a>) -> Self {
        Error::Tokens(err)
    }
}

impl<'a> Error<'a> {
    pub(crate) fn unexpected_token<T: IntoIterator<Item = TokenKind>>(
        expected_kinds: T,
        tokens: &PeekableTokens<'a>,
        token: &Token<'a>,
    ) -> Error<'a> {
        Error::UnexpectedToken {
            expected_kinds: expected_kinds.into_iter().collect(),
            kind: token.kind,
            cursor: token.range.start,
            text: tokens.text(),
        }
    }
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { expected_kinds, kind, cursor, text } => {
                let lines = text.split('\n');
                let mut read_len: usize = 0;
                for (line_number, line) in lines.enumerate() {
                    let len = line.len();
                    if read_len + len < *cursor {
                        read_len += len + 1;
                        continue;
                    }
                    let line_cursor = *cursor - read_len;
                    let expected_kinds = expected_kinds.iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                        .join(" / ");

                    write!(f, "Unexpected token found. line: {}\n", line_number + 1)?;
                    write!(f, "Expected: {}\n", expected_kinds)?;
                    write!(f, "Found: {}\n\n", kind.to_string())?;
                    write!(f, "{}\n", line)?;
                    write!(f, "{}^\n", " ".repeat(line_cursor - 1))?;
                    break;
                }
                
                return Ok(())
            },
            Error::Tokens(err) => err.fmt(f),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

pub(crate) type Result<'a, T> = std::result::Result<T, Error<'a>>;
