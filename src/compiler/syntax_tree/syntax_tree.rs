use super::{ExpressionNode, FunctionNode, StatementNode, UnaryOperator, error::{Result, Error}, LocalValue};
use super::super::tokens::{PeekableTokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) struct SyntaxTree<'a> {
    pub(crate) root: FunctionNode<'a>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::function(&mut tokens)?;
        Ok(SyntaxTree { root })
    }

    fn function(tokens: &mut PeekableTokens<'a>) -> Result<'a, FunctionNode<'a>> {
        let mut statements: Vec<StatementNode<'a>> = Vec::new();
        let mut local_values: Vec<LocalValue<'a>> = Vec::new();

        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Return => return Ok(FunctionNode {
                    local_values,
                    statements,
                    return_statement: Some(SyntaxTree::return_statement(tokens)?),
                }),
                TokenKind::EOF => return Ok(FunctionNode { local_values, statements, return_statement: None }),
                _ => statements.push(SyntaxTree::statement(tokens, &mut local_values)?),
            }
        }
    }

    fn statement(tokens: &mut PeekableTokens<'a>, local_values: &mut Vec<LocalValue<'a>>) -> Result<'a, StatementNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::Identifier => SyntaxTree::assign(tokens, local_values),
            _ => Ok(StatementNode::Expression(SyntaxTree::end_of_statement(tokens)?)),
        }
    }

    fn assign(tokens: &mut PeekableTokens<'a>, local_values: &mut Vec<LocalValue<'a>>) -> Result<'a, StatementNode<'a>> {
        let token = tokens.next()?;
        if token.kind != TokenKind::Identifier {
            return Err(Error::unexpected_token([TokenKind::Identifier], tokens, &token));
        }
        
        let identifier = token.value;
        local_values.push(LocalValue { name: identifier });

        let token = tokens.next()?;
        if token.kind != TokenKind::Assign {
            return Err(Error::unexpected_token([TokenKind::Assign], tokens, &token));
        }

        let expression = SyntaxTree::end_of_statement(tokens)?;
        Ok(StatementNode::Assign(identifier, expression))
    }

    fn return_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let token = tokens.next()?;
        if token.kind != TokenKind::Return {
            return Err(Error::unexpected_token([TokenKind::Return], tokens, &token));
        }
        
        SyntaxTree::end_of_statement(tokens)
    }

    fn end_of_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let expression = SyntaxTree::expression(tokens)?;
        let token = tokens.next()?;
        if token.kind != TokenKind::NewLine {
            Err(Error::unexpected_token([TokenKind::NewLine], tokens, &token))
        } else {
            Ok(expression)
        }
    }

    fn expression(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::add(tokens)
    }

    fn add(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let mut tree = SyntaxTree::multiply(tokens)?;
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::multiply(tokens)?;
                    tree = ExpressionNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() }
                },
                _ => return Ok(tree),
            }
        }
    }
    
    fn multiply(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let mut tree = SyntaxTree::unary(tokens)?;
        
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Multiply | TokenKind::Divide => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::unary(tokens)?;
                    tree = ExpressionNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
                },
                _ => return Ok(tree),
            }
        }
    }

    fn unary(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::Plus => {
                tokens.next()?;
                SyntaxTree::primary(tokens)
            },
            TokenKind::Minus => {
                tokens.next()?;
                Ok(ExpressionNode::UnaryExpr {
                    child: Box::new(SyntaxTree::primary(tokens)?),
                    operator: UnaryOperator::Minus,
                })
            },
            _ => SyntaxTree::primary(tokens),
        }
    }

    fn primary(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::OpenParen => {
                tokens.next()?;
                let tree = SyntaxTree::expression(tokens)?;
                
                let token = tokens.next()?;
                if token.kind != TokenKind::CloseParen {
                    Err(Error::unexpected_token([TokenKind::CloseParen], tokens, &token))
                } else {
                    Ok(tree)
                }
            },
            TokenKind::Number => Ok(ExpressionNode::Value(tokens.next()?.value)),
            TokenKind::Identifier => Ok(ExpressionNode::Identifier(tokens.next()?.value)),
            _ => {
                let token = tokens.next()?;
                Err(Error::unexpected_token([TokenKind::OpenParen, TokenKind::Number], tokens, &token))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PeekableTokens, SyntaxTree, ExpressionNode, FunctionNode, StatementNode};
    use super::super::BinaryOperator;

    #[test]
    fn local_value() {
        SyntaxTree::new(PeekableTokens::new("foo = 2\nbar = 3")).unwrap();
    }

    #[test]
    fn it_works() {
        assert_eq!(
            SyntaxTree::new(PeekableTokens::new("2 + 3 * (5 - (1 + 4)) / 2\n")).unwrap(),
            SyntaxTree {
                root: FunctionNode {
                    local_values: vec![],
                    return_statement: None,
                    statements: vec![
                        StatementNode::Expression(
                            ExpressionNode::BinaryExpr {
                                lhs: Box::new(ExpressionNode::Value("2")),
                                rhs: Box::new(ExpressionNode::BinaryExpr {
                                    lhs: Box::new(ExpressionNode::BinaryExpr {
                                        lhs: Box::new(ExpressionNode::Value("3")),
                                        rhs: Box::new(ExpressionNode::BinaryExpr {
                                            lhs: Box::new(ExpressionNode::Value("5")),
                                            rhs: Box::new(ExpressionNode::BinaryExpr {
                                                lhs: Box::new(ExpressionNode::Value("1")),
                                                rhs: Box::new(ExpressionNode::Value("4")),
                                                operator: BinaryOperator::Plus,
                                            }),
                                            operator: BinaryOperator::Minus,
                                        }),
                                        operator: BinaryOperator::Multiply,
                                    }),
                                    rhs: Box::new(ExpressionNode::Value("2")),
                                    operator: BinaryOperator::Divide,
                                }),
                                operator: BinaryOperator::Plus,
                            }
                        )
                    ]
                }
            }
        )
    }

    fn error_test(text: &str, expected: &str) {
        let err = SyntaxTree::new(PeekableTokens::new(text)).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }

    #[test]
    fn error() {
        let expected = r#"Unexpected token found. line: 1
Expected: '(' / number

2 + 3 * (5 - (1 + +)) / 2
                  ^
"#;
        error_test("2 + 3 * (5 - (1 + +)) / 2", expected);
    }
}
