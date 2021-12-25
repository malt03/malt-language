use std::{collections::{HashMap, HashSet}, iter::FromIterator};
use super::{Token, TokenKind, error::{Error, Result}};

#[derive(Debug)]
pub(crate) struct TokenConverter {
    tokens_maps: Vec<(usize, HashMap<String, TokenKind>)>,
    newline_chars: HashSet<char>,
    identifier_chars: HashSet<char>,
    type_chars: HashSet<char>,
    number_chars: HashSet<char>,
    whitespaces: HashSet<char>,
}

impl TokenConverter {
    pub(crate) fn new() -> Self {
        let six: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("return".into(), TokenKind::Return),
        ]);
        let five: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("false".into(), TokenKind::False),
            ("elsif".into(), TokenKind::Elsif),
        ]);
        let four: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("true".into(), TokenKind::True),
            ("else".into(), TokenKind::Else),
        ]);
        let two: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("==".into(), TokenKind::Equal),
            ("!=".into(), TokenKind::NotEqual),
            (">=".into(), TokenKind::GreaterOrEqual),
            ("<=".into(), TokenKind::LessOrEqual),
            ("fn".into(), TokenKind::Function),
            ("if".into(), TokenKind::If),
        ]);
        let one: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("(".into(), TokenKind::OpenParen),
            (")".into(), TokenKind::CloseParen),
            ("{".into(), TokenKind::OpenBrace),
            ("}".into(), TokenKind::CloseBrace),
            ("+".into(), TokenKind::Plus),
            ("-".into(), TokenKind::Minus),
            ("*".into(), TokenKind::Multiply),
            ("/".into(), TokenKind::Divide),
            ("=".into(), TokenKind::Assign),
            (">".into(), TokenKind::Greater),
            ("<".into(), TokenKind::Less),
            (":".into(), TokenKind::Colon),
            (",".into(), TokenKind::Comma),
        ]);
        
        TokenConverter {
            tokens_maps: vec![
                (6, six),
                (5, five),
                (4, four),
                (2, two),
                (1, one),
            ],
            newline_chars: HashSet::from_iter(['\n']),
            identifier_chars: HashSet::from_iter(vec!['_'..='_', 'a'..='z', '0'..='9'].into_iter().flatten()),
            type_chars: HashSet::from_iter(vec!['a'..='z', 'A'..='Z', '0'..='9'].into_iter().flatten()),
            number_chars: HashSet::from_iter('0'..='9'),
            whitespaces: HashSet::from_iter([' ', '\t']),
        }
    }

    pub(crate) fn convert<'a>(&self, text: &'a str, cursor: &mut usize) -> Result<'a, Token<'a>> {
        let max_length = text.len();

        loop {
            let current_char = &text[*cursor..].chars().next().unwrap();
            
            if self.whitespaces.contains(current_char) {
                *cursor += current_char.len_utf8();
                continue;
            }
            
            let start = *cursor;

            if self.newline_chars.contains(current_char) {
                *cursor += current_char.len_utf8();
                return Ok(Token::new(TokenKind::NewLine, text, start..*cursor));
            }
            
            if let Some(kind) = self.convert_operators(text, cursor, max_length) {
                return Ok(Token::new(kind, text, start..*cursor));
            }
            
            return if self.number_chars.contains(current_char) {
                let has_period = self.take_number(text, cursor);
                if has_period {
                    Ok(Token::new(TokenKind::Double, text, start..*cursor))
                } else {
                    Ok(Token::new(TokenKind::Int, text, start..*cursor))
                }
            } else if self.identifier_chars.contains(current_char) {
                self.take_identifier(text, cursor);
                Ok(Token::new(TokenKind::Identifier, text, start..*cursor))
            } else if self.type_chars.contains(current_char) {
                self.take_type(text, cursor);
                Ok(Token::new(TokenKind::Type, text, start..*cursor))
            } else {
                Err(Error::unexpected_char(start, text))
            }
        }
    }

    fn take_while<'a, P: FnMut(char) -> bool>(text: &'a str, cursor: &mut usize, mut predicate: P) -> &'a str {
        let chars = text[*cursor..].chars();
        let mut len: usize = 0;
        for char in chars.into_iter() {
            if !predicate(char) { break; }
            len += char.len_utf8();
        }
        let start = *cursor;
        *cursor += len;
        
        return &text[start..*cursor];
    }

    fn take_identifier<'a>(&self, text: &'a str, cursor: &mut usize) {
        TokenConverter::take_while(text, cursor, |c| self.identifier_chars.contains(&c));
    }

    fn take_type<'a>(&self, text: &'a str, cursor: &mut usize) {
        TokenConverter::take_while(text, cursor, |c| self.type_chars.contains(&c));
    }

    fn take_number<'a>(&self, text: &'a str, cursor: &mut usize) -> bool {
        let mut has_period = false;
        TokenConverter::take_while(text, cursor, |c| {
            if self.number_chars.contains(&c) { return true }
            if c == '.' {
                if has_period { return false }
                has_period = true;
                return true;
            }
            false
        });
        has_period
    }

    fn convert_operators<'a>(&self, text: &'a str, cursor: &mut usize, max_length: usize) -> Option<TokenKind> {
        for (length, map) in &self.tokens_maps {
            if *cursor + *length > max_length { continue; }
            let target: String = text[*cursor..].chars().take(*length).collect();
            if let Some(kind) = map.get(&target) {
                *cursor += target.len();
                return Some(kind.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::TokenKind;
    use super::TokenConverter;

    fn test(text: &str, cursor: usize, expect_cursor: usize, token: Token) {
        let converter = TokenConverter::new();
        let mut cursor = cursor;
        assert_eq!(converter.convert(text, &mut cursor).unwrap(), token);
        assert_eq!(cursor, expect_cursor);
    }

    #[test]
    fn it_works() {
        test("<=foo", 0, 2, Token::new(TokenKind::LessOrEqual, "<=", 0..2));
        test("foo>bar", 3, 4, Token::new(TokenKind::Greater, ">", 3..4));
        test("foo > bar", 5, 9, Token::new(TokenKind::Identifier, "bar", 6..9));
        test("foo=", 3, 4, Token::new(TokenKind::Assign, "=", 3..4));
        test("f3oo=", 0, 4, Token::new(TokenKind::Identifier, "f3oo", 0..4));
        test("32foo=", 0, 2, Token::new(TokenKind::Int, "32", 0..2));
    }
}
