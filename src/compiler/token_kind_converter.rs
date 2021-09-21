use std::{collections::HashMap, iter::FromIterator};
use super::token_kind::TokenKind;

pub(crate) struct TokenKindConverter<'a> {
  maps: Vec<(usize, HashMap<&'a str, TokenKind<'a>>)>,
}

impl<'a> TokenKindConverter<'a> {
  pub(crate) fn new() -> Self {
    let two = HashMap::<_, _>::from_iter([
      ("==", TokenKind::Equal),
      (">=", TokenKind::GreaterOrEqual),
      ("<=", TokenKind::LessOrEqual),
    ]);
    let one = HashMap::<_, _>::from_iter([
      ("(", TokenKind::OpenParen),
      (")", TokenKind::CloseParen),
      ("+", TokenKind::Plus),
      ("-", TokenKind::Minus),
      ("*", TokenKind::Times),
      ("/", TokenKind::Divide),
      ("=", TokenKind::Assign),
      (">", TokenKind::Greater),
      ("<", TokenKind::Less),
    ]);
    
    TokenKindConverter {
      maps: vec![
        (2, two),
        (1, one),
      ]
    }
  }

  pub(crate) fn convert(&self, text: &str, cursor: &mut usize) -> Option<&TokenKind> {
    for (length, map) in &self.maps {
      let target = &text[*cursor..(*cursor+length)];
      if let Some(kind) = map.get(target) {
        *cursor += length;
        return Some(kind);
      }
    }
    return None;
  }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;
    use super::TokenKindConverter;

    fn test(text: &str, before_cursor: usize, expect_kind: Option<&TokenKind>, expect_cursor: usize) {
      let converter = TokenKindConverter::new();
      let mut cursor = before_cursor;
      let kind = converter.convert(text, &mut cursor);
      assert_eq!(kind, expect_kind);
      assert_eq!(cursor, expect_cursor);
    }

    #[test]
    fn it_works() {
      test("<=foo", 0, Some(&TokenKind::LessOrEqual), 2);
      test("foo>bar", 3, Some(&TokenKind::Greater), 4);
      test("foo>", 3, Some(&TokenKind::Greater), 4);
    }
}
