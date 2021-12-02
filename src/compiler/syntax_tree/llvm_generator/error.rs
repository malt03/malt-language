#[derive(Debug)]
pub enum Error<'a> {
    ValueNotFound(&'a str)
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ValueNotFound(name) => write!(f, "cannot find value `{}` in this scope", name),
        }
    }
}

pub(crate) type Result<'a, T> = core::result::Result<T, Error<'a>>;
