use std::iter::Peekable;
use super::BinaryOperator;
use super::super::tokens::{Tokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) enum SyntaxTree<'a> {
  Value(&'a str),
  BinaryExpr {
    lhs: Box<SyntaxTree<'a>>,
    rhs: Box<SyntaxTree<'a>>,
    operator: BinaryOperator,
  },
}

impl<'a> SyntaxTree<'a> {
  pub(crate) fn new(tokens: &mut Peekable<Tokens<'a>>) -> SyntaxTree<'a> {
    SyntaxTree::expr(tokens)
  }
  
  fn expr(tokens: &mut Peekable<Tokens<'a>>) -> SyntaxTree<'a> {
    let mut tree = SyntaxTree::mul(tokens);
    
    loop {
      let token = tokens.peek().unwrap();
      match token.kind {
        TokenKind::Plus | TokenKind::Minus => {
          let token = tokens.next().unwrap();
          let rhs = SyntaxTree::mul(tokens);
          tree = SyntaxTree::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() }
        },
        _ => return tree,
      }
    }
  }
  
  fn mul(tokens: &mut Peekable<Tokens<'a>>) -> SyntaxTree<'a> {
    let mut tree = SyntaxTree::primary(tokens);
    
    loop {
      let token = tokens.peek().unwrap();
      match token.kind {
        TokenKind::Times | TokenKind::Divide => {
          let token = tokens.next().unwrap();
          let rhs = SyntaxTree::primary(tokens);
          tree = SyntaxTree::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
        },
        _ => return tree,
      }
    }
  }
  
  fn primary(tokens: &mut Peekable<Tokens<'a>>) -> SyntaxTree<'a> {
    let token = tokens.peek().unwrap();

    if token.kind == TokenKind::OpenParen {
      tokens.next();
      let tree = SyntaxTree::expr(tokens);
      if tokens.next().unwrap().kind != TokenKind::CloseParen { panic!("unexpected") }
      return tree;
    }

    if token.kind == TokenKind::Number {
      let token = tokens.next().unwrap();
      return SyntaxTree::Value(token.value);
    }

    panic!("unexpected")
  }
}
