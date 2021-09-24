use super::{UnaryOperator, BinaryOperator, error::{Result, Error}};
use super::super::tokens::{PeekableTokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) enum SyntaxTreeNode<'a> {
    Value(&'a str),
    UnaryExpr {
        child: Box<SyntaxTreeNode<'a>>,
        operator: UnaryOperator,
    },
    BinaryExpr {
        lhs: Box<SyntaxTreeNode<'a>>,
        rhs: Box<SyntaxTreeNode<'a>>,
        operator: BinaryOperator,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct SyntaxTree<'a> {
    pub(crate) root: SyntaxTreeNode<'a>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::expr(&mut tokens)?;
        Ok(SyntaxTree { root })
    }
    
    fn expr(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTreeNode<'a>> {
        let mut tree = SyntaxTree::mul(tokens)?;
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::mul(tokens)?;
                    tree = SyntaxTreeNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() }
                },
                _ => return Ok(tree),
            }
        }
    }
    
    fn mul(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTreeNode<'a>> {
        let mut tree = SyntaxTree::unary(tokens)?;
        
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Times | TokenKind::Divide => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::unary(tokens)?;
                    tree = SyntaxTreeNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
                },
                _ => return Ok(tree),
            }
        }
    }

    fn primary(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTreeNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::OpenParen => {
                let token = tokens.next()?;
                let tree = SyntaxTree::expr(tokens)?;
                
                if tokens.next()?.kind != TokenKind::CloseParen {
                    Err(Error::unexpected_token([TokenKind::CloseParen], tokens, &token))
                } else {
                    Ok(tree)
                }
            },
            TokenKind::Number => Ok(SyntaxTreeNode::Value(tokens.next()?.value)),
            _ => {
                let token = tokens.next()?;
                Err(Error::unexpected_token([TokenKind::OpenParen, TokenKind::Number], tokens, &token))
            },
        }
    }

    fn unary(tokens: &mut PeekableTokens<'a>) -> Result<'a, SyntaxTreeNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::Plus => {
                tokens.next()?;
                SyntaxTree::primary(tokens)
            },
            TokenKind::Minus => {
                tokens.next()?;
                Ok(SyntaxTreeNode::UnaryExpr {
                    child: Box::new(SyntaxTree::primary(tokens)?),
                    operator: UnaryOperator::Minus,
                })
            },
            _ => SyntaxTree::primary(tokens),
        }
    }
}
