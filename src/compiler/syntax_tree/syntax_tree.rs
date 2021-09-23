use super::{BinaryOperator, error::{Result, Error}};
use super::super::tokens::{PeekableTokens, TokenKind};

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
  pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
    SyntaxTree::expr(&mut tokens)
  }
  
  fn expr(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
    let mut tree = SyntaxTree::mul(tokens)?;
    
    loop {
      let token = tokens.peek();
      match token.kind {
        TokenKind::Plus | TokenKind::Minus => {
          let token = tokens.next();
          let rhs = SyntaxTree::mul(tokens)?;
          tree = SyntaxTree::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() }
        },
        _ => return Ok(tree),
      }
    }
  }
  
  fn mul(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
    let mut tree = SyntaxTree::primary(tokens)?;
    
    loop {
      let token = tokens.peek();
      match token.kind {
        TokenKind::Times | TokenKind::Divide => {
          let token = tokens.next();
          let rhs = SyntaxTree::primary(tokens)?;
          tree = SyntaxTree::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
        },
        _ => return Ok(tree),
      }
    }
  }
  
  fn primary(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
    let token = tokens.peek();

    match token.kind {
      TokenKind::OpenParen => {
        tokens.next();
        let tree = SyntaxTree::expr(tokens)?;
        
        if tokens.next().kind != TokenKind::CloseParen {
          Err(Error::unexpected_token([TokenKind::CloseParen], tokens))
        } else {
          Ok(tree)
        }
      },
      TokenKind::Number => Ok(SyntaxTree::Value(tokens.next().value)),
      _ => Err(Error::unexpected_token([TokenKind::OpenParen, TokenKind::Number], tokens))
    }
  }
}
