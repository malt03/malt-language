use super::super::tokens::{self, PeekableTokens, TokenKind, Token};

#[derive(Debug)]
pub enum Error<'a> {
    UnexpectedToken { expected_kinds: Vec<TokenKind>, kind: TokenKind, cursor: usize, text: &'a str },
    DuplicatedName { name: &'a str, cursor: usize, text: &'a str },
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

    pub(crate) fn duplicated_name(
        name: &'a str,
        tokens: &PeekableTokens<'a>,
        token: &Token<'a>,
    ) -> Error<'a> {
        Error::DuplicatedName {
            name,
            cursor: token.range.start,
            text: tokens.text(),
        }
    }
}

fn line_error<Handler: Fn(usize, &mut std::fmt::Formatter) -> std::fmt::Result>(
    f: &mut std::fmt::Formatter,
    cursor: &usize,
    text: &str,
    handler: Handler,
) -> std::fmt::Result {
    let lines = text.split('\n');
    let mut read_len: usize = 0;
    for (line_number, line) in lines.enumerate() {
        let len = line.len();
        if read_len + len < *cursor {
            read_len += len + 1;
            continue;
        }
        handler(line_number, f)?;

        let line_cursor = *cursor - read_len;
        f.write_fmt(format_args!("{}\n", line))?;
        f.write_fmt(format_args!("{}^\n", " ".repeat(line_cursor)))?;

        break;
    }
    Ok(())
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { expected_kinds, kind, cursor, text } =>
                line_error(f, cursor, text, |line_number, f| {
                    let expected_kinds = expected_kinds.iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                        .join(" / ");

                    f.write_fmt(format_args!("Unexpected token found. line: {}\n", line_number + 1))?;
                    f.write_fmt(format_args!("Expected: {}\n", expected_kinds))?;
                    f.write_fmt(format_args!("Found: {}\n\n", kind.to_string()))?;
                    
                    Ok(())
                }),
            Error::DuplicatedName { name, cursor, text } => 
                line_error(f, cursor, text, |line_number, f| 
                    f.write_fmt(format_args!("'{}' is already used. line: {}\n", name, line_number + 1))
                ),
            Error::Tokens(err) => err.fmt(f),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

pub(crate) type Result<'a, T> = std::result::Result<T, Error<'a>>;
