use super::super::super::tokens::Token;

#[derive(Debug)]
pub enum Error<'a> {
    ValueNotFound { name: &'a str, cursor: usize, text: &'a str },
    FunctionNotFound { name: &'a str, cursor: usize, text: &'a str },
}

impl<'a> Error<'a> {
    pub(crate) fn value_not_found(
        token: &Token<'a>,
    ) -> Error<'a> {
        Error::ValueNotFound {
            name: token.value(),
            cursor: token.range.start,
            text: token.text,
        }
    }

    pub(crate) fn function_not_found(
        token: &Token<'a>,
    ) -> Error<'a> {
        Error::FunctionNotFound {
            name: token.value(),
            cursor: token.range.start,
            text: token.text,
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
        handler(line_number + 1, f)?;

        let line_cursor = *cursor - read_len;
        write!(f, "{}\n", line)?;
        write!(f, "{}^\n", " ".repeat(line_cursor))?;

        break;
    }
    Ok(())
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ValueNotFound{name, cursor, text} => 
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "cannot find value `{}` in this scope. line: {}\n", name, line_number)
                }),
            Error::FunctionNotFound{name, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "cannot find function `{}` in this scope. line: {}\n", name, line_number)
                }),
        }
    }
}

pub(crate) type Result<'a, T> = core::result::Result<T, Error<'a>>;
