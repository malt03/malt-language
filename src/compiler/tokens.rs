mod token;
mod token_kind;
mod token_converter;

use token::Token;
pub(crate) use token_kind::TokenKind;
use token_converter::TokenConverter;

pub(crate) struct Tokens<'a> {
  converter: TokenConverter<'a>,
  text: &'a str,
  cursor: usize,
}

impl<'a> std::fmt::Debug for Tokens<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Tokens").field("text", &self.text).field("cursor", &self.cursor).finish()
  }
}

impl<'a> Tokens<'a> {
  pub(crate) fn new(text: &'a str) -> Tokens {
    Tokens {
      converter: TokenConverter::new(),
      text,
      cursor: 0,
    }
  }
}

impl<'a> Iterator for Tokens<'a> {
  type Item = Token<'a>;
  
  fn next(&mut self) -> Option<Token<'a>> {
    if self.text.len() == self.cursor { return Some(Token::new(TokenKind::EOF, "", 0..0)); }

    Some(self.converter.convert(self.text, &mut self.cursor))
  }
}

#[cfg(test)]
mod tests {
  use super::{Tokens, Token, TokenKind};

  fn test<'a, Iter: IntoIterator<Item = Token<'a>>>(text: &str, expect: Iter) {
    assert_eq!(Tokens::new(text).collect::<Vec<_>>(), expect.into_iter().collect::<Vec<_>>());
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
}
