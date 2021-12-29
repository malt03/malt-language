use super::super::super::tokens::Token;

#[derive(Debug)]
pub enum Error<'a> {
    ValueNotFound { name: &'a str, cursor: usize, text: &'a str },
    FunctionNotFound { name: &'a str, cursor: usize, text: &'a str },
    TypeNotFound { name: &'a str, cursor: usize, text: &'a str },
    UnexpectedType { expected_type: &'a str, type_: &'a str, cursor: usize, text: &'a str },
    UnexpectedArgumentsLength { expected_length: usize, length: usize, cursor: usize, text: &'a str },
    UnexpectedLabel { expected_label: &'a str, label: &'a str, cursor: usize, text: &'a str },
    AllReturn { cursor: usize, text: &'a str },
    UnexpectedPropertiesLength { expected_length: usize, length: usize, cursor: usize, text: &'a str },
    NoField { struct_name: &'a str, field: &'a str, cursor: usize, text: &'a str },
}

impl<'a> Error<'a> {
    pub(crate) fn value_not_found(token: &Token<'a>) -> Error<'a> {
        Error::ValueNotFound { name: token.value(), cursor: token.range.start, text: token.text }
    }

    pub(crate) fn function_not_found(token: &Token<'a>) -> Error<'a> {
        Error::FunctionNotFound { name: token.value(), cursor: token.range.start, text: token.text }
    }

    pub(crate) fn type_not_found(token: &Token<'a>) -> Error<'a> {
        Error::TypeNotFound { name: token.value(), cursor: token.range.start, text: token.text }
    }

    pub(crate) fn unexpected_type(expected_type: &'a str, type_: &'a str, token: &Token<'a>) -> Error<'a> {
        Error::UnexpectedType { expected_type, type_, cursor: token.range.start, text: token.text }
    }

    pub(crate) fn unexpected_arguments_length(expected_length: usize, length: usize, token: &Token<'a>) -> Error<'a> {
        Error::UnexpectedArgumentsLength { expected_length, length, cursor: token.range.start, text: token.text }
    }

    pub(crate) fn unexpected_label(expected_label: &'a str, label: &Token<'a>) -> Error<'a> {
        Error::UnexpectedLabel { expected_label, label: label.value(), cursor: label.range.start, text: label.text }
    }

    pub(crate) fn all_return(token: &Token<'a>) -> Error<'a> {
        Error::AllReturn { cursor: token.range.start, text: token.text }
    }

    pub(crate) fn unexpected_properties_length(expected_length: usize, length: usize, token: &Token<'a>) -> Error<'a> {
        Error::UnexpectedPropertiesLength { expected_length, length, cursor: token.range.start, text: token.text }
    }

    pub(crate) fn no_field(struct_name: &'a str, field: &Token<'a>) -> Error<'a> {
        Error::NoField { struct_name, field: field.value(), cursor: field.range.start, text: field.text }
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
            Error::TypeNotFound{name, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "cannot find type `{}` in this scope. line: {}\n", name, line_number)
                }),
            Error::UnexpectedType{expected_type, type_, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "unexpected type found. line: {}\n", line_number)?;
                    write!(f, "Expected: {}\n", expected_type)?;
                    write!(f, "Found: {}\n", type_)?;
                    Ok(())
                }),
            Error::UnexpectedArgumentsLength{expected_length, length, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "unexpected arguments length. line: {}\n", line_number)?;
                    write!(f, "Expected: {}\n", expected_length)?;
                    write!(f, "Found: {}\n", length)?;
                    Ok(())
                }),
            Error::UnexpectedLabel{expected_label, label, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "unexpected label found. line: {}\n", line_number)?;
                    write!(f, "Expected: {}\n", expected_label)?;
                    write!(f, "Found: {}\n", label)?;
                    Ok(())
                }),
            Error::AllReturn{cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "Return in all branches. line: {}\n", line_number)?;
                    write!(f, "Instead, write `return if {{ a }} else {{ b }}`.\n")?;
                    Ok(())
                }),
            Error::UnexpectedPropertiesLength{expected_length, length, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| {
                    write!(f, "unexpected properties length. line: {}\n", line_number)?;
                    write!(f, "Expected: {}\n", expected_length)?;
                    write!(f, "Found: {}\n", length)?;
                    Ok(())
                }),
            Error::NoField{struct_name, field, cursor, text} =>
                line_error(f, cursor, text, |line_number, f| 
                    write!(f, "no field `{}` on type `{}`. line: {}\n", field, struct_name, line_number)
                ),
        }
    }
}

pub(crate) type Result<'a, T> = core::result::Result<T, Error<'a>>;
