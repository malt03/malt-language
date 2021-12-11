use super::{
    ExpressionNode,
    FunctionNode,
    ModuleNode,
    StatementNode,
    UnaryOperator,
    binary_operator::BinaryOperator,
    ValueDefinitionNode,
    ReturnNode,
    CallArgumentNode,
    error::{Result, Error},
};
use super::super::tokens::{Token, PeekableTokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) struct SyntaxTree<'a> {
    pub(crate) text: &'a str,
    pub(crate) root: ModuleNode<'a>,
}

struct FunctionBody<'a> {
    statements: Vec<StatementNode<'a>>,
    ret: Option<ReturnNode<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::module(&mut tokens)?;
        Ok(SyntaxTree { text: tokens.text(), root })
    }

    fn skip_newlines(tokens: &mut PeekableTokens<'a>) -> Result<'a, ()> {
        while tokens.peek()?.kind == TokenKind::NewLine {
            tokens.next()?;
        }
        Ok(())
    }

    fn confirm_kind(kind: TokenKind, token: &Token<'a>, tokens: &PeekableTokens<'a>) -> Result<'a, ()> {
        if token.kind == kind {
            Ok(())
        } else {
            Err(Error::unexpected_token([kind], tokens, &token))
        }
    }

    fn module(tokens: &mut PeekableTokens<'a>) -> Result<'a, ModuleNode<'a>> {
        let mut functions: Vec<FunctionNode<'a>> = Vec::new();
        loop {
            SyntaxTree::skip_newlines(tokens)?;

            functions.push(SyntaxTree::function(tokens)?);

            let token = tokens.peek()?;
            if token.kind == TokenKind::EOF { return Ok(ModuleNode{ functions }); }
        }
    }

    fn function(tokens: &mut PeekableTokens<'a>) -> Result<'a, FunctionNode<'a>> {
        SyntaxTree::confirm_kind(TokenKind::Function, &tokens.next()?, tokens)?;

        let name = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &name, tokens)?;

        SyntaxTree::confirm_kind(TokenKind::OpenParen, &tokens.next()?, tokens)?;
        SyntaxTree::skip_newlines(tokens)?;

        let arguments = SyntaxTree::arguments(tokens)?;
        SyntaxTree::skip_newlines(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;

        let return_type = if tokens.peek()?.kind == TokenKind::Colon {
            tokens.next()?;
            let token = tokens.next()?;
            SyntaxTree::confirm_kind(TokenKind::Type, &token, tokens)?;
            Some(token)
        } else { None };

        SyntaxTree::skip_newlines(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::OpenBrace, &tokens.next()?, tokens)?;
        SyntaxTree::skip_newlines(tokens)?;

        let body = SyntaxTree::function_body(tokens)?;
        
        let close = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::CloseBrace, &close, tokens)?;
        SyntaxTree::confirm_kind(TokenKind::NewLine, &tokens.next()?, tokens)?;

        Ok(FunctionNode {
            name,
            arguments,
            return_type,
            statements: body.statements,
            ret: body.ret,
            close,
        })
    }

    fn arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, Vec<ValueDefinitionNode<'a>>> {
        if tokens.peek()?.kind != TokenKind::Identifier { return Ok(Vec::new()); }

        let value = SyntaxTree::value_definition(tokens)?;
        let mut values: Vec<ValueDefinitionNode<'a>> = vec![value];
        while tokens.peek()?.kind == TokenKind::Comma {
            tokens.next()?;
            SyntaxTree::skip_newlines(tokens)?;
            let value = SyntaxTree::value_definition(tokens)?;
            values.push(value);
        }
        Ok(values)
    }

    fn value_definition(tokens: &mut PeekableTokens<'a>) -> Result<'a, ValueDefinitionNode<'a>> {
        let name = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &name, tokens)?;
        let colon = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Colon, &colon, tokens)?;
        let typ = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Type, &typ, tokens)?;
        
        Ok(ValueDefinitionNode { name, typ })
    }

    fn function_body(tokens: &mut PeekableTokens<'a>) -> Result<'a, FunctionBody<'a>> {
        let mut statements: Vec<StatementNode<'a>> = Vec::new();

        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Return => {
                    let token = tokens.next()?;
                    SyntaxTree::confirm_kind(TokenKind::Return, &token, tokens)?;
                    let expression = SyntaxTree::end_of_statement(tokens)?;
            
                    return Ok(FunctionBody { statements, ret: Some(ReturnNode { token, expression }) });
                },
                TokenKind::CloseBrace => {
                    return Ok(FunctionBody { statements, ret: None });
                },
                _ => statements.push(SyntaxTree::statement(tokens)?),
            }
        }
    }

    fn statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, StatementNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::Identifier => SyntaxTree::assign(tokens),
            _ => Ok(StatementNode::Expression(SyntaxTree::end_of_statement(tokens)?)),
        }
    }

    fn assign(tokens: &mut PeekableTokens<'a>) -> Result<'a, StatementNode<'a>> {
        let lhs = SyntaxTree::value_definition(tokens)?;

        SyntaxTree::confirm_kind(TokenKind::Assign, &tokens.next()?, tokens)?;

        let rhs = SyntaxTree::end_of_statement(tokens)?;
        let statement = StatementNode::Assign { lhs, rhs };

        Ok(statement)
    }

    fn end_of_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let expression = SyntaxTree::expression(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::NewLine, &tokens.next()?, tokens)?;
        Ok(expression)
    }

    fn expression(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::add(tokens)
    }

    fn add(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let mut expression = SyntaxTree::multiply(tokens)?;
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let token = tokens.next()?;
                    SyntaxTree::skip_newlines(tokens)?;
                    let rhs = SyntaxTree::multiply(tokens)?;
                    let operator: BinaryOperator = (&token.kind).into();
                    expression = ExpressionNode::BinaryExpr {
                        lhs: Box::new(expression),
                        rhs: Box::new(rhs),
                        operator,
                        token,
                    };
                },
                _ => return Ok(expression),
            }
        }
    }
    
    fn multiply(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let mut expression = SyntaxTree::unary(tokens)?;
        
        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Multiply | TokenKind::Divide => {
                    let token = tokens.next()?;
                    SyntaxTree::skip_newlines(tokens)?;
                    let rhs = SyntaxTree::unary(tokens)?;
                    let operator: BinaryOperator = (&token.kind).into();
                    expression = ExpressionNode::BinaryExpr {
                        lhs: Box::new(expression),
                        rhs: Box::new(rhs),
                        operator,
                        token,
                    };
                },
                _ => return Ok(expression),
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
                let token = tokens.next()?;
                Ok(ExpressionNode::UnaryExpr {
                    token,
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
                SyntaxTree::skip_newlines(tokens)?;
                let expression = SyntaxTree::expression(tokens)?;
                
                SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;
                SyntaxTree::skip_newlines(tokens)?;

                Ok(expression)
            },
            TokenKind::Int => {
                let token = tokens.next()?;
                Ok(ExpressionNode::Int(token))
            },
            TokenKind::Double => {
                let token = tokens.next()?;
                Ok(ExpressionNode::Double(token))
            },
            TokenKind::True => {
                let token = tokens.next()?;
                Ok(ExpressionNode::Bool(true, token))
            },
            TokenKind::False => {
                let token = tokens.next()?;
                Ok(ExpressionNode::Bool(false, token))
            },
            TokenKind::Identifier => {
                let token = tokens.next()?;
                if tokens.peek()?.kind == TokenKind::OpenParen {
                    Ok(SyntaxTree::function_call(tokens, token)?)
                }  else {
                    Ok(ExpressionNode::Identifier(token))
                }
            },
            _ => {
                let token = tokens.next()?;
                Err(Error::unexpected_token([
                    TokenKind::OpenParen,
                    TokenKind::Int,
                    TokenKind::Double,
                    TokenKind::True,
                    TokenKind::False,
                    TokenKind::Identifier,
                ], tokens, &token))
            },
        }
    }

    fn function_call(tokens: &mut PeekableTokens<'a>, token: Token<'a>) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::confirm_kind(TokenKind::OpenParen, &tokens.next()?, tokens)?;
        let arguments: Vec<CallArgumentNode<'a>> = if tokens.peek()?.kind == TokenKind::CloseParen {
            tokens.next()?;
            Vec::new()
        } else {
            let arguments = SyntaxTree::call_arguments(tokens)?;
            SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;
            arguments
        };
        
        Ok(ExpressionNode::FunctionCall {token, arguments })
    }

    fn call_arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, Vec<CallArgumentNode<'a>>> {
        SyntaxTree::skip_newlines(tokens)?;
        let mut arguments = vec![SyntaxTree::call_argument(tokens)?];

        while tokens.peek()?.kind == TokenKind::Comma {
            tokens.next()?;
            SyntaxTree::skip_newlines(tokens)?;
            if tokens.peek()?.kind == TokenKind::CloseParen { break; }
            arguments.push(SyntaxTree::call_argument(tokens)?);
        }

        Ok(arguments)
    }

    fn call_argument(tokens: &mut PeekableTokens<'a>) -> Result<'a, CallArgumentNode<'a>> {
        let label = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &label, tokens)?;
        let colon = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Colon, &colon, tokens)?;
        let value = SyntaxTree::expression(tokens)?;
        
        Ok(CallArgumentNode { label, value })
    }
}

#[cfg(test)]
mod tests {
    use super::{PeekableTokens, SyntaxTree};

    fn error_test(text: &str, expected: &str) {
        let err = SyntaxTree::new(PeekableTokens::new(text)).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }

    #[test]
    fn error() {
        let code = r#"fn main() {
    foo: I32 = 2 + 3 * ((5 - 1) + 1) / 3
    bar: I32 = 10 + ++2
    return foo + bar
}
"#;
        let expected = r#"Unexpected token found. line: 3
Expected: '(' / number
Found: '+'

    bar: I32 = 10 + ++2
                     ^
"#;
        error_test(code, expected);
    }
}
