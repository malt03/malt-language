use super::{
    ExpressionNode,
    FunctionNode,
    BlockNode,
    ModuleNode,
    StatementNode,
    UnaryOperator,
    binary_operator::{BinaryOperator, CompareOperator},
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

fn create_name_with_arguments<'a, I: Iterator<Item = String>>(name: &Token<'a>, arguments: I) -> String {
    let arguments_name = arguments.collect::<Vec<_>>().join("-");
    if arguments_name.is_empty() {
        name.value().to_string()
    } else {
        format!("{}-{}", name.value(), arguments_name)
    }
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::module(&mut tokens)?;
        Ok(SyntaxTree { text: tokens.text(), root })
    }

    fn skip_newlines(tokens: &mut PeekableTokens<'a>) -> Result<'a, ()> {
        while tokens.peek(0)?.kind == TokenKind::NewLine {
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
            SyntaxTree::skip_newlines(tokens)?;

            let token = tokens.peek(0)?;
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

        let return_type = if tokens.peek(0)?.kind == TokenKind::Colon {
            tokens.next()?;
            let token = tokens.next()?;
            SyntaxTree::confirm_kind(TokenKind::Type, &token, tokens)?;
            Some(token)
        } else { None };

        SyntaxTree::skip_newlines(tokens)?;

        let block = SyntaxTree::block(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::NewLine, &tokens.next()?, tokens)?;

        let name_with_arguments = create_name_with_arguments(
            &name,
            arguments.iter().map(|arg| arg.name.value().to_string()),
        );

        Ok(FunctionNode {
            name,
            arguments,
            name_with_arguments,
            return_type,
            block,
        })
    }

    fn arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, Vec<ValueDefinitionNode<'a>>> {
        if tokens.peek(0)?.kind != TokenKind::Identifier { return Ok(Vec::new()); }

        let value = SyntaxTree::value_definition(tokens)?;
        let mut values: Vec<ValueDefinitionNode<'a>> = vec![value];
        while tokens.peek(0)?.kind == TokenKind::Comma {
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

    fn block(tokens: &mut PeekableTokens<'a>) -> Result<'a, BlockNode<'a>> {
        SyntaxTree::confirm_kind(TokenKind::OpenBrace, &tokens.next()?, tokens)?;

        let mut ret = None;
        let mut statements = Vec::new();
        loop {
            SyntaxTree::skip_newlines(tokens)?;
            let token = tokens.peek(0)?;
            match token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Return => {
                    ret = Some(SyntaxTree::ret(tokens)?);
                    break;
                },
                _ => statements.push(SyntaxTree::statement(tokens)?),
            }
            
            let token = tokens.peek(0)?;
            if token.kind != TokenKind::NewLine { break; }
        }
        SyntaxTree::skip_newlines(tokens)?;
        let close = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::CloseBrace, &close, tokens)?;

        Ok(BlockNode { statements, ret, close })
    }

    fn ret(tokens: &mut PeekableTokens<'a>) -> Result<'a, ReturnNode<'a>> {
        let token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Return, &token, tokens)?;
        let kind = tokens.peek(0)?.kind;
        if kind == TokenKind::NewLine || kind == TokenKind::CloseBrace {
            Ok(ReturnNode { token, expression: None })
        } else {
            let expression = SyntaxTree::expression(tokens)?;
            Ok(ReturnNode { token, expression: Some(expression) })
        }
    }

    fn statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, StatementNode<'a>> {
        if SyntaxTree::is_assign(tokens)? {
            SyntaxTree::assign(tokens)
        } else {
            Ok(StatementNode::Expression(SyntaxTree::expression(tokens)?))
        }
    }

    fn is_assign(tokens: &mut PeekableTokens<'a>) -> Result<'a, bool> {
        if tokens.peek(0)?.kind != TokenKind::Identifier { return Ok(false); }
        if tokens.peek(1)?.kind == TokenKind::Colon { return Ok(true); }
        if tokens.peek(1)?.kind == TokenKind::Assign { return Ok(true); }
        Ok(false)
    }

    fn assign(tokens: &mut PeekableTokens<'a>) -> Result<'a, StatementNode<'a>> {
        let name = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &name, tokens)?;

        let typ = if tokens.peek(0)?.kind == TokenKind::Colon {
            tokens.next()?;
            let typ = tokens.next()?;
            SyntaxTree::confirm_kind(TokenKind::Type, &typ, tokens)?;
            Some(typ)
        } else { None };

        SyntaxTree::confirm_kind(TokenKind::Assign, &tokens.next()?, tokens)?;

        let rhs = SyntaxTree::expression(tokens)?;
        Ok(StatementNode::Assign { name, typ, rhs })
    }

    fn expression(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::equality(tokens)
    }

    fn equality(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let lhs = SyntaxTree::relational(tokens)?;
        
        let token = tokens.peek(0)?;
        match token.kind {
            TokenKind::Equal | TokenKind::NotEqual => {
                let token = tokens.next()?;
                SyntaxTree::skip_newlines(tokens)?;
                let rhs = SyntaxTree::relational(tokens)?;
                let operator: CompareOperator = (&token.kind).into();
                Ok(ExpressionNode::CompareExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator,
                    token,
                })
            },
            _ => Ok(lhs),
        }
    }

    fn relational(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let lhs = SyntaxTree::add(tokens)?;
        
        let token = tokens.peek(0)?;
        match token.kind {
            TokenKind::Greater | TokenKind::GreaterOrEqual | TokenKind::Less | TokenKind::LessOrEqual => {
                let token = tokens.next()?;
                SyntaxTree::skip_newlines(tokens)?;
                let rhs = SyntaxTree::add(tokens)?;
                let operator: CompareOperator = (&token.kind).into();
                Ok(ExpressionNode::CompareExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator,
                    token,
                })
            },
            _ => Ok(lhs),
        }
    }

    fn add(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let mut expression = SyntaxTree::multiply(tokens)?;
        loop {
            let token = tokens.peek(0)?;
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
            let token = tokens.peek(0)?;
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

    fn if_branch(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let if_token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::If, &if_token, tokens)?;
        let if_branch = (Box::new(SyntaxTree::expression(tokens)?), Box::new(SyntaxTree::block(tokens)?));
        SyntaxTree::skip_newlines(tokens)?;
        
        let mut if_branches: Vec<(Box<ExpressionNode<'a>>, Box<BlockNode<'a>>)> = vec![if_branch];
        
        while tokens.peek(0)?.kind == TokenKind::Elsif {
            tokens.next()?;
            if_branches.push((Box::new(SyntaxTree::expression(tokens)?), Box::new(SyntaxTree::block(tokens)?)));
            SyntaxTree::skip_newlines(tokens)?;
        }

        let else_branch = if tokens.peek(0)?.kind == TokenKind::Else {
            tokens.next()?;
            Some(Box::new(SyntaxTree::block(tokens)?))
        } else { None };

        Ok(ExpressionNode::IfBranch { token: if_token, if_branches, else_branch })
    }

    fn unary(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        let token = tokens.peek(0)?;
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
        let token = tokens.peek(0)?;
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
                if tokens.peek(0)?.kind == TokenKind::OpenParen {
                    Ok(SyntaxTree::function_call(tokens, token)?)
                }  else {
                    Ok(ExpressionNode::Identifier(token))
                }
            },
            TokenKind::If => SyntaxTree::if_branch(tokens),
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
        let arguments: Vec<CallArgumentNode<'a>> = if tokens.peek(0)?.kind == TokenKind::CloseParen {
            tokens.next()?;
            Vec::new()
        } else {
            let arguments = SyntaxTree::call_arguments(tokens)?;
            SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;
            arguments
        };
        
        let name_with_arguments = create_name_with_arguments(
            &token,
            arguments.iter().map(|arg| arg.label.value().to_string()),
        );

        Ok(ExpressionNode::FunctionCall {token, arguments, name_with_arguments})
    }

    fn call_arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, Vec<CallArgumentNode<'a>>> {
        SyntaxTree::skip_newlines(tokens)?;
        let mut arguments = vec![SyntaxTree::call_argument(tokens)?];

        while tokens.peek(0)?.kind == TokenKind::Comma {
            tokens.next()?;
            SyntaxTree::skip_newlines(tokens)?;
            if tokens.peek(0)?.kind == TokenKind::CloseParen { break; }
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
