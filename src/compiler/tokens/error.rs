#[derive(Debug)]
pub enum Error<'a> {
    UnexpectedChar { cursor: usize, text: &'a str },
}

impl<'a> Error<'a> {
    pub(crate) fn unexpected_char(
        cursor: usize,
        text: &'a str,
    ) -> Error<'a> { Error::UnexpectedChar { cursor, text } }
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedChar { cursor, text } => {
                let lines = text.split('\n');
                let mut read_len: usize = 0;
                for (line_number, line) in lines.enumerate() {
                    let len = line.len();
                    if read_len + len < *cursor {
                        read_len += len + 1;
                        continue;
                    }
                    let line_cursor = *cursor - read_len;

                    write!(f, "unexpected character found. line: {}\n\n", line_number + 1)?;
                    write!(f, "{}\n", line)?;
                    write!(f, "{}^\n", " ".repeat(line_cursor))?;
                    break;
                }
                
                return Ok(())
            },
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

pub(crate) type Result<'a, T> = std::result::Result<T, Error<'a>>;
