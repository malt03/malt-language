#[derive(Debug)]
pub enum Error<'a> {
    ValueNotFound{name: &'a str, cursor: usize, text: &'a str}
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ValueNotFound{name, cursor, text} => {
                let lines = text.split('\n');
                let mut read_len: usize = 0;
                for (line_number, line) in lines.enumerate() {
                    let len = line.len();
                    if read_len + len < *cursor {
                        read_len += len + 1;
                        continue;
                    }
                    let line_cursor = *cursor - read_len;

                    write!(f, "cannot find value `{}` in this scope. line: {}\n", name, line_number)?;
                    write!(f, "{}\n", line)?;
                    write!(f, "{}^\n", " ".repeat(line_cursor))?;
                    break;
                }
                
                Ok(())
            },
        }
    }
}

pub(crate) type Result<'a, T> = core::result::Result<T, Error<'a>>;
