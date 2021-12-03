use super::{Node, ExpressionNode, FunctionNode, StatementNode, UnaryOperator, error::{Result, Error}, LocalValue};
use super::super::tokens::{PeekableTokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) struct SyntaxTree<'a> {
    pub(crate) root: Node<'a, FunctionNode<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::function(&mut tokens)?;
        Ok(SyntaxTree { root })
    }

    fn function(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, FunctionNode<'a>>> {
        let mut statements: Vec<Node<'a, StatementNode<'a>>> = Vec::new();
        let mut local_values: Vec<LocalValue<'a>> = Vec::new();
        let text = tokens.text();

        loop {
            let token = tokens.peek()?;
            let cursor = token.range.start;
            match &token.kind {
                TokenKind::Return => {
                    let kind = FunctionNode {
                        local_values,
                        statements,
                        return_statement: Some(SyntaxTree::return_statement(tokens)?),
                    };
                    return Ok(Node::new(kind, cursor, text));
                },
                TokenKind::EOF => {
                    let kind = FunctionNode { local_values, statements, return_statement: None };
                    return Ok(Node::new(kind, cursor, text));
                },
                _ => statements.push(SyntaxTree::statement(tokens, &mut local_values)?),
            }
        }
    }

    fn statement(tokens: &mut PeekableTokens<'a>, local_values: &mut Vec<LocalValue<'a>>) -> Result<'a, Node<'a, StatementNode<'a>>> {
        let text = tokens.text();
        let token = tokens.peek()?;
        let cursor = token.range.start;

        match token.kind {
            TokenKind::Identifier => SyntaxTree::assign(tokens, local_values),
            _ => {
                let kind = StatementNode::Expression(SyntaxTree::end_of_statement(tokens)?);
                Ok(Node::new(kind, cursor, text))
            },
        }
    }

    fn assign(tokens: &mut PeekableTokens<'a>, local_values: &mut Vec<LocalValue<'a>>) -> Result<'a, Node<'a, StatementNode<'a>>> {
        let token = tokens.next()?;
        
        let text = tokens.text();
        let cursor = token.range.start;

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
        let kind = StatementNode::Assign(identifier, expression);
        
        Ok(Node::new(kind, cursor, text))
    }

    fn return_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let token = tokens.next()?;
        if token.kind != TokenKind::Return {
            return Err(Error::unexpected_token([TokenKind::Return], tokens, &token));
        }
        
        SyntaxTree::end_of_statement(tokens)
    }

    fn end_of_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let expression = SyntaxTree::expression(tokens)?;
        let token = tokens.next()?;
        if token.kind != TokenKind::NewLine {
            Err(Error::unexpected_token([TokenKind::NewLine], tokens, &token))
        } else {
            Ok(expression)
        }
    }

    fn expression(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        SyntaxTree::add(tokens)
    }

    fn add(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let text = tokens.text();

        let mut tree = SyntaxTree::multiply(tokens)?;
        loop {
            let token = tokens.peek()?;
            let cursor = token.range.start;

            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::multiply(tokens)?;
                    let kind = ExpressionNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
                    tree = Node::new(kind, cursor, text);
                },
                _ => return Ok(tree),
            }
        }
    }
    
    fn multiply(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let text = tokens.text();

        let mut tree = SyntaxTree::unary(tokens)?;
        
        loop {
            let token = tokens.peek()?;
            let cursor = token.range.start;
            match token.kind {
                TokenKind::Multiply | TokenKind::Divide => {
                    let token = tokens.next()?;
                    let rhs = SyntaxTree::unary(tokens)?;
                    let kind = ExpressionNode::BinaryExpr { lhs: Box::new(tree), rhs: Box::new(rhs), operator: (&token.kind).into() };
                    tree = Node::new(kind, cursor, text);
                },
                _ => return Ok(tree),
            }
        }
    }

    fn unary(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let text = tokens.text();
        let token = tokens.peek()?;
        let cursor = token.range.start;

        match token.kind {
            TokenKind::Plus => {
                tokens.next()?;
                SyntaxTree::primary(tokens)
            },
            TokenKind::Minus => {
                tokens.next()?;
                let kind = ExpressionNode::UnaryExpr {
                    child: Box::new(SyntaxTree::primary(tokens)?),
                    operator: UnaryOperator::Minus,
                };
                Ok(Node::new(kind, cursor, text))
            },
            _ => SyntaxTree::primary(tokens),
        }
    }

    fn primary(tokens: &mut PeekableTokens<'a>) -> Result<'a, Node<'a, ExpressionNode<'a>>> {
        let text = tokens.text();
        let token = tokens.peek()?;
        let cursor = token.range.start;

        match token.kind {
            TokenKind::OpenParen => {
                tokens.next()?;
                let tree = SyntaxTree::expression(tokens)?;
                
                let token = tokens.next()?;
                if token.kind != TokenKind::CloseParen {
                    // Err(Error::unexpected_token([TokenKind::CloseParen], &tokens, &token))
                    unimplemented!()
                } else {
                    Ok(tree)
                }
            },
            TokenKind::Number => Ok(Node::new(ExpressionNode::Value(tokens.next()?.value), cursor, text)),
            TokenKind::Identifier => Ok(Node::new(ExpressionNode::Identifier(tokens.next()?.value), cursor, text)),
            _ => {
                let token = tokens.next()?;
                Err(Error::unexpected_token([TokenKind::OpenParen, TokenKind::Number], tokens, &token))
            },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::{PeekableTokens, SyntaxTree, ExpressionNode, FunctionNode, StatementNode};
//     use super::super::BinaryOperator;

//     #[test]
//     fn local_value() {
//         SyntaxTree::new(PeekableTokens::new("foo = 2\nbar = 3")).unwrap();
//     }

//     #[test]
//     fn it_works() {
//         assert_eq!(
//             SyntaxTree::new(PeekableTokens::new("2 + 3 * (5 - (1 + 4)) / 2\n")).unwrap(),
//             SyntaxTree {
//                 root: FunctionNode {
//                     local_values: vec![],
//                     return_statement: None,
//                     statements: vec![
//                         StatementNode::Expression(
//                             ExpressionNode::BinaryExpr {
//                                 lhs: Box::new(ExpressionNode::Value("2")),
//                                 rhs: Box::new(ExpressionNode::BinaryExpr {
//                                     lhs: Box::new(ExpressionNode::BinaryExpr {
//                                         lhs: Box::new(ExpressionNode::Value("3")),
//                                         rhs: Box::new(ExpressionNode::BinaryExpr {
//                                             lhs: Box::new(ExpressionNode::Value("5")),
//                                             rhs: Box::new(ExpressionNode::BinaryExpr {
//                                                 lhs: Box::new(ExpressionNode::Value("1")),
//                                                 rhs: Box::new(ExpressionNode::Value("4")),
//                                                 operator: BinaryOperator::Plus,
//                                             }),
//                                             operator: BinaryOperator::Minus,
//                                         }),
//                                         operator: BinaryOperator::Multiply,
//                                     }),
//                                     rhs: Box::new(ExpressionNode::Value("2")),
//                                     operator: BinaryOperator::Divide,
//                                 }),
//                                 operator: BinaryOperator::Plus,
//                             }
//                         )
//                     ]
//                 }
//             }
//         )
//     }

//     fn error_test(text: &str, expected: &str) {
//         let err = SyntaxTree::new(PeekableTokens::new(text)).unwrap_err();
//         assert_eq!(err.to_string(), expected);
//     }

//     #[test]
//     fn error() {
//         let expected = r#"Unexpected token found. line: 1
// Expected: '(' / number

// 2 + 3 * (5 - (1 + +)) / 2
//                   ^
// "#;
//         error_test("2 + 3 * (5 - (1 + +)) / 2", expected);
//     }
// }
