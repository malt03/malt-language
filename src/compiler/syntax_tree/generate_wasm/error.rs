use std::io;

use super::super::super::tokens::Token;

#[derive(Debug)]
pub enum Error<'a> {
    FunctionNotFound { name: &'a str, cursor: usize, text: &'a str },
    IO(io::Error),
}

impl<'a> Error<'a> {
    pub(crate) fn function_not_found(name: &'a str, text: &'a str, token: &Token<'a>) -> Error<'a> {
        Error::FunctionNotFound { name, cursor: token.range.start, text }
    }
}

impl<'a> From<io::Error> for Error<'a> {
    fn from(err: io::Error) -> Self { Error::IO(err) }
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FunctionNotFound { name: _, cursor: _, text: _ } => f.write_str(""),
            Error::IO(err) => err.fmt(f),
        }
    }
}
