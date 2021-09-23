use std::{collections::{HashMap, HashSet}, iter::FromIterator};
use super::{Token, TokenKind};

#[derive(Debug)]
pub(crate) struct TokenConverter {
    tokens_maps: Vec<(usize, HashMap<String, TokenKind>)>,
    newline_chars: HashSet<char>,
    identifier_chars: HashSet<char>,
    number_chars: HashSet<char>,
    whitespaces: HashSet<char>,
}

impl TokenConverter {
    pub(crate) fn new() -> Self {
        let two: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("==".into(), TokenKind::Equal),
            (">=".into(), TokenKind::GreaterOrEqual),
            ("<=".into(), TokenKind::LessOrEqual),
        ]);
        let one: HashMap::<String, TokenKind> = HashMap::from_iter([
            ("(".into(), TokenKind::OpenParen),
            (")".into(), TokenKind::CloseParen),
            ("+".into(), TokenKind::Plus),
            ("-".into(), TokenKind::Minus),
            ("*".into(), TokenKind::Times),
            ("/".into(), TokenKind::Divide),
            ("=".into(), TokenKind::Assign),
            (">".into(), TokenKind::Greater),
            ("<".into(), TokenKind::Less),
        ]);
        
        TokenConverter {
            tokens_maps: vec![
                (2, two),
                (1, one),
            ],
            newline_chars: HashSet::from_iter(['\n']),
            identifier_chars: HashSet::from_iter(vec!['_'..='_', 'a'..='z', 'A'..='Z', '0'..='9'].into_iter().flatten()),
            number_chars: HashSet::from_iter('0'..='9'),
            whitespaces: HashSet::from_iter([' ', '\t']),
        }
    }

    pub(crate) fn convert<'a>(&self, text: &'a str, cursor: &mut usize) -> Token<'a> {
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
                return Token::new(TokenKind::NewLine, "\n", start..*cursor);
            }
            
            if let Some((kind, value)) = self.convert_operators(text, cursor, max_length) {
                return Token::new(kind, value, start..*cursor);
            }
            
            return if self.number_chars.contains(current_char) {
                Token::new(TokenKind::Number, self.take_number(text, cursor), start..*cursor)
            } else if self.identifier_chars.contains(current_char) {
                Token::new(TokenKind::Identifier, self.take_identifier(text, cursor), start..*cursor)
            } else {
                panic!("unexpected character {}", current_char)
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

    fn take_identifier<'a>(&self, text: &'a str, cursor: &mut usize) -> &'a str {
        TokenConverter::take_while(text, cursor, |c| self.identifier_chars.contains(&c))
    }

    fn take_number<'a>(&self, text: &'a str, cursor: &mut usize) -> &'a str {
        TokenConverter::take_while(text, cursor, |c| self.number_chars.contains(&c))
    }

    fn convert_operators<'a>(&self, text: &'a str, cursor: &mut usize, max_length: usize) -> Option<(TokenKind, &'a str)> {
        for (length, map) in &self.tokens_maps {
            if *cursor + *length > max_length { continue; }
            let target: String = text[*cursor..].chars().take(*length).collect();
            if let Some(kind) = map.get(&target) {
                let start = *cursor;
                *cursor += target.len();
                return Some((kind.clone(), &text[start..*cursor]));
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
        assert_eq!(converter.convert(text, &mut cursor), token);
        assert_eq!(cursor, expect_cursor);
    }

    #[test]
    fn it_works() {
        test("<=foo", 0, 2, Token::new(TokenKind::LessOrEqual, "<=", 0..2));
        test("foo>bar", 3, 4, Token::new(TokenKind::Greater, ">", 3..4));
        test("foo > bar", 5, 9, Token::new(TokenKind::Identifier, "bar", 6..9));
        test("foo=", 3, 4, Token::new(TokenKind::Assign, "=", 3..4));
        test("f3oo=", 0, 4, Token::new(TokenKind::Identifier, "f3oo", 0..4));
        test("32foo=", 0, 2, Token::new(TokenKind::Number, "32", 0..2));
    }
}
