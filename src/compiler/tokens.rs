use super::{Token, TokenConverter};

pub(crate) struct Tokens<'a> {
  converter: TokenConverter<'a>,
  text: &'a str,
  cursor: usize,
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

impl<'a> Iterator for Tokens<'a> {
  type Item = Token<'a>;
  
  fn next(&mut self) -> Option<Token<'a>> {
    if self.text.len() == self.cursor { return None; }

    Some(self.converter.convert(self.text, &mut self.cursor))
  }
}

mod tests {
  use super::super::{Tokens, Token, TokenKind};

  fn test<'a, Iter: IntoIterator<Item = Token<'a>>>(text: &str, expect: Iter) {
    assert_eq!(Tokens::new(text).collect::<Vec<_>>(), expect.into_iter().collect::<Vec<_>>());
  }
  
  #[test]
  fn it_works() {
    test("aa * bbb", [
      Token::new(TokenKind::Identifier, "aa", 0..2),
      Token::new(TokenKind::Times, "*", 3..4),
      Token::new(TokenKind::Identifier, "bbb", 5..8),
    ]);
  }
}
