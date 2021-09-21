use std::{collections::{HashMap, HashSet}, iter::FromIterator};
use super::token_kind::TokenKind;

pub(crate) struct TokenKindConverter<'a> {
  maps: Vec<(usize, HashMap<&'a str, TokenKind>)>,
  identifier_chars: HashSet<char>,
  number_chars: HashSet<char>,
  whitespaces: HashSet<char>,
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
      ],
      identifier_chars: HashSet::from_iter(vec!['_'..='_', 'a'..='z', 'A'..='Z', '0'..='9'].into_iter().flatten()),
      number_chars: HashSet::from_iter('0'..='9'),
      whitespaces: HashSet::from_iter([' ', '\t']),
    }
  }

  pub(crate) fn convert<'b>(&self, text: &'b str, cursor: &mut usize) -> (TokenKind, &'b str) {
    let max_length = text.len();

    loop {
      if *cursor == max_length { return (TokenKind::NewLine, ""); }
      
      let current_char = &text[*cursor..(*cursor + 1)].chars().next().unwrap();
      if self.whitespaces.contains(current_char) {
        *cursor += 1;
        continue;
      }
      
      if let Some(val) = self.convert_operators(text, cursor, max_length) {
        return val;
      }
      
      return match current_char {
        '_' | 'a'..='z' | 'A'..='Z' => (TokenKind::Identifier, self.take_identifier(text, cursor)),
        '0'..='9' => (TokenKind::Number, self.take_number(text, cursor)),
        other => panic!("unexpected character {}", other),
      }
    }
  }

  fn take_while<'b, P: FnMut(char) -> bool>(text: &'b str, cursor: &mut usize, mut predicate: P) -> &'b str {
    let chars = text[*cursor..].chars();
    let mut count: usize = 0;
    for char in chars.into_iter() {
      if !predicate(char) { break; }
      count += 1;
    }
    let start = *cursor;
    *cursor += count;
    
    return &text[start..*cursor];
  }

  fn take_identifier<'b>(&self, text: &'b str, cursor: &mut usize) -> &'b str {
    TokenKindConverter::take_while(text, cursor, |c| self.identifier_chars.contains(&c))
  }

  fn take_number<'b>(&self, text: &'b str, cursor: &mut usize) -> &'b str {
    TokenKindConverter::take_while(text, cursor, |c| self.number_chars.contains(&c))
  }

  fn convert_operators<'b>(&self, text: &'b str, cursor: &mut usize, max_length: usize) -> Option<(TokenKind, &'b str)> {
    for (length, map) in &self.maps {
      if *cursor + length > max_length { continue; }
      let target = &text[*cursor..(*cursor + length)];
      if let Some(kind) = map.get(target) {
        *cursor += length;
        return Some((kind.clone(), target));
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;
    use super::TokenKindConverter;

    fn test(text: &str, before_cursor: usize, expect_kind: TokenKind, expect_value: &str, expect_cursor: usize) {
      let converter = TokenKindConverter::new();
      let mut cursor = before_cursor;
      let kind = converter.convert(text, &mut cursor);
      assert_eq!(kind, (expect_kind, expect_value));
      assert_eq!(cursor, expect_cursor);
    }

    #[test]
    fn it_works() {
      test("<=foo", 0, TokenKind::LessOrEqual, "<=", 2);
      test("foo>bar", 3, TokenKind::Greater, ">", 4);
      test("foo=", 3, TokenKind::Assign, "=", 4);
      test("foo=", 0, TokenKind::Identifier, "foo", 3);
    }
}
