mod error;
mod token;
mod token_kind;
mod token_converter;

use error::Result;
pub(crate) use error::Error;
use token::Token;
pub(crate) use token_kind::TokenKind;
use token_converter::TokenConverter;

#[derive(Debug)]
pub(crate) struct PeekableTokens<'a> {
    tokens: Tokens<'a>,
    peeked: Option<Token<'a>>,
}

impl<'a> PeekableTokens<'a> {
    pub(crate) fn new(text: &'a str) -> PeekableTokens {
        PeekableTokens { tokens: Tokens::new(text), peeked: None }
    }
}

impl<'a> PeekableTokens<'a> {
    pub(crate) fn next(&mut self) -> Result<'a, Token<'a>> {
        match self.peeked.take() {
            Some(v) => Ok(v),
            None => self.tokens.next(),
        }
    }

    pub(crate) fn peek(&mut self) -> Result<'a, &Token<'a>> {
        let tokens = &mut self.tokens;
        match self.peeked {
            Some(ref token) => Ok(token),
            None => {
                self.peeked = Some(tokens.next()?);
                Ok(self.peeked.as_ref().unwrap())
            }
        }
    }

    pub(crate) fn text(&self) -> &'a str { self.tokens.text }
    pub(crate) fn cursor(&self) -> usize { self.tokens.cursor }
}

impl<'a> Iterator for PeekableTokens<'a> {
    type Item = Result<'a, Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next();

        match &next {
            Ok(token) => {
                if token.kind == TokenKind::EOF { None }
                else { Some(next) }
            },
            Err(_) => Some(next),
        }
    }
}

struct Tokens<'a> {
    converter: TokenConverter,
    text: &'a str,
    cursor: usize,
}

impl<'a> std::fmt::Debug for Tokens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokens").field("text", &self.text).field("cursor", &self.cursor).finish()
    }
}

impl<'a> Tokens<'a> {
    fn new(text: &'a str) -> Tokens {
        Tokens {
            converter: TokenConverter::new(),
            text,
            cursor: 0,
        }
    }
}

impl<'a> Tokens<'a> {
    fn next(&mut self) -> Result<'a, Token<'a>> {
        if self.text.len() == self.cursor { return Ok(Token::new(TokenKind::EOF, "", 0..0)); }

        self.converter.convert(self.text, &mut self.cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::{PeekableTokens, Token, TokenKind};

    fn test<'a, Iter: IntoIterator<Item = Token<'a>>>(text: &str, expect: Iter) {
        assert_eq!(
            PeekableTokens::new(text).map(|r| r.unwrap()).collect::<Vec<_>>(),
            expect.into_iter().collect::<Vec<_>>(),
        );
    }
    
    #[test]
    fn it_works() {
        test("aa * bbb\nprint(3)", [
            Token::new(TokenKind::Identifier, "aa", 0..2),
            Token::new(TokenKind::Times, "*", 3..4),
            Token::new(TokenKind::Identifier, "bbb", 5..8),
            Token::new(TokenKind::NewLine, "\n", 8..9),
            Token::new(TokenKind::Identifier, "print", 9..14),
            Token::new(TokenKind::OpenParen, "(", 14..15),
            Token::new(TokenKind::Number, "3", 15..16),
            Token::new(TokenKind::CloseParen, ")", 16..17 ),
        ]);
    }

    #[test]
    fn error() {
        let mut tokens = PeekableTokens::new("hoge¥piyo");
        tokens.next().unwrap();

        let expected = r#"unexpected character found. line: 1

hoge¥piyo
    ^
"#;
        assert_eq!(tokens.next().unwrap_err().to_string(), expected);
    }
}
