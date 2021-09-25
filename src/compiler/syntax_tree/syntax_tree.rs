use std::{collections::HashMap, iter::FromIterator};

use super::{ModuleNode, ExpressionNode, FunctionNode, StatementNode, Return, UnaryOperator, error::{Result, Error}, LocalValue};
use super::super::tokens::{Token, PeekableTokens, TokenKind};

#[derive(Debug, PartialEq)]
pub(crate) struct SyntaxTree<'a> {
    pub(crate) root: ModuleNode<'a>,
}

struct FunctionBody<'a> {
    statements: Vec<StatementNode<'a>>,
    return_: Option<Return<'a>>,
    local_values: HashMap<&'a str, LocalValue<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new(mut tokens: PeekableTokens<'a>) -> Result<'a, SyntaxTree<'a>> {
        let root = SyntaxTree::module(&mut tokens)?;
        Ok(SyntaxTree { root })
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
        let mut functions: HashMap<&'a str, FunctionNode<'a>> = HashMap::new();
        loop {
            SyntaxTree::skip_newlines(tokens)?;

            let (function, name_token) = SyntaxTree::function(tokens)?;
            if functions.contains_key(function.name) { return Err(Error::duplicated_name(function.name, tokens, &name_token)); }
            functions.insert(function.name, function);

            let token = tokens.peek()?;
            if token.kind == TokenKind::EOF { return Ok(ModuleNode{ functions }); }
        }
    }

    fn function(tokens: &mut PeekableTokens<'a>) -> Result<'a, (FunctionNode<'a>, Token<'a>)> {
        SyntaxTree::confirm_kind(TokenKind::Function, &tokens.next()?, tokens)?;

        let name_token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &name_token, tokens)?;
        let name = name_token.value;

        SyntaxTree::confirm_kind(TokenKind::OpenParen, &tokens.next()?, tokens)?;
        SyntaxTree::skip_newlines(tokens)?;

        let (arguments, arguments_map) = SyntaxTree::arguments(tokens)?;
        SyntaxTree::skip_newlines(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;

        let return_type = if tokens.peek()?.kind == TokenKind::Colon {
            tokens.next()?;
            let token = tokens.next()?;
            SyntaxTree::confirm_kind(TokenKind::Type, &token, tokens)?;
            token.value
        } else { "Void" };

        SyntaxTree::skip_newlines(tokens)?;
        SyntaxTree::confirm_kind(TokenKind::OpenBrace, &tokens.next()?, tokens)?;
        SyntaxTree::skip_newlines(tokens)?;

        let body = SyntaxTree::function_body(tokens, return_type)?;

        SyntaxTree::confirm_kind(TokenKind::CloseBrace, &tokens.next()?, tokens)?;
        SyntaxTree::confirm_kind(TokenKind::NewLine, &tokens.next()?, tokens)?;
        
        Ok((
            FunctionNode {
                name,
                arguments,
                arguments_map,
                local_values: body.local_values,
                statements: body.statements,
                return_: body.return_,
            },
            name_token,
        ))
    }

    fn arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, (Vec<LocalValue<'a>>, HashMap<&'a str, LocalValue<'a>>)> {
        if tokens.peek()?.kind != TokenKind::Identifier { return Ok((Vec::new(), HashMap::new())); }

        let (value, _) = SyntaxTree::value_definition(tokens)?;
        let mut values: Vec<LocalValue<'a>> = vec![value.clone()];
        let mut values_map: HashMap<&'a str, LocalValue<'a>> = HashMap::from_iter([(value.name, value)]);
        while tokens.peek()?.kind == TokenKind::Comma {
            tokens.next()?;
            SyntaxTree::skip_newlines(tokens)?;
            let (value, token) = SyntaxTree::value_definition(tokens)?;
            if values_map.contains_key(value.name) {
                return Err(Error::duplicated_name(value.name, tokens, &token));
            }
            values.push(value.clone());
            values_map.insert(value.name, value);
        }
        Ok((values, values_map))
    }

    fn value_definition(tokens: &mut PeekableTokens<'a>) -> Result<'a, (LocalValue<'a>, Token<'a>)> {
        let token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Identifier, &token, tokens)?;
        let name = token.value;
        let token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Colon, &token, tokens)?;
        let token = tokens.next()?;
        SyntaxTree::confirm_kind(TokenKind::Type, &token, tokens)?;
        let type_ = token.value;
        
        Ok((LocalValue { name, type_ }, token))
    }

    fn function_body(tokens: &mut PeekableTokens<'a>, return_type: &'a str) -> Result<'a, FunctionBody<'a>> {
        let mut statements: Vec<StatementNode<'a>> = Vec::new();
        let mut local_values: HashMap<&'a str, LocalValue<'a>> = HashMap::new();

        loop {
            let token = tokens.peek()?;
            match token.kind {
                TokenKind::Return => {
                    return if return_type == "Void" {
                        let token = tokens.next()?;
                        Err(Error::unexpected_token([TokenKind::CloseBrace], tokens, &token))
                    } else {
                        Ok(FunctionBody {
                            local_values,
                            statements,
                            return_: Some(Return { expression: SyntaxTree::return_statement(tokens)?, type_: return_type }),
                        })
                    }
                },
                TokenKind::CloseBrace => {
                    return if return_type == "Void" {
                        Ok(FunctionBody { local_values, statements, return_: None })
                    } else {
                        let token = tokens.next()?;
                        Err(Error::unexpected_token([TokenKind::Return], tokens, &token))
                    }
                    
                },
                _ => statements.push(SyntaxTree::statement(tokens, &mut local_values)?),
            }
        }
    }

    fn statement(tokens: &mut PeekableTokens<'a>, local_values: &mut HashMap<&'a str, LocalValue<'a>>) -> Result<'a, StatementNode<'a>> {
        let token = tokens.peek()?;
        match token.kind {
            TokenKind::Identifier => SyntaxTree::assign(tokens, local_values),
            _ => Ok(StatementNode::Expression(SyntaxTree::end_of_statement(tokens)?)),
        }
    }

    fn assign(tokens: &mut PeekableTokens<'a>, local_values: &mut HashMap<&'a str, LocalValue<'a>>) -> Result<'a, StatementNode<'a>> {
        let (value, token) = SyntaxTree::value_definition(tokens)?;

        SyntaxTree::confirm_kind(TokenKind::Assign, &tokens.next()?, tokens)?;

        let expression = SyntaxTree::end_of_statement(tokens)?;
        let statement = StatementNode::Assign(value.name, expression);
        
        if local_values.contains_key(value.name) { return Err(Error::duplicated_name(value.name, tokens, &token)); }
        local_values.insert(value.name, value);

        Ok(statement)
    }

    fn return_statement(tokens: &mut PeekableTokens<'a>) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::confirm_kind(TokenKind::Return, &tokens.next()?, tokens)?;
        
        SyntaxTree::end_of_statement(tokens)
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
                    expression = ExpressionNode::BinaryExpr { lhs: Box::new(expression), rhs: Box::new(rhs), operator: (&token.kind).into() }
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
                    expression = ExpressionNode::BinaryExpr { lhs: Box::new(expression), rhs: Box::new(rhs), operator: (&token.kind).into() };
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
                SyntaxTree::skip_newlines(tokens)?;
                let expression = SyntaxTree::expression(tokens)?;
                
                SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;
                SyntaxTree::skip_newlines(tokens)?;

                Ok(expression)
            },
            TokenKind::Number => Ok(ExpressionNode::Value(tokens.next()?.value)),
            TokenKind::Identifier => {
                let name = tokens.next()?.value;
                if tokens.peek()?.kind == TokenKind::OpenParen {
                    Ok(SyntaxTree::function_call(tokens, name)?)
                }  else {
                    Ok(ExpressionNode::Identifier(name))
                }
            },
            _ => {
                let token = tokens.next()?;
                Err(Error::unexpected_token([TokenKind::OpenParen, TokenKind::Number], tokens, &token))
            },
        }
    }

    fn function_call(tokens: &mut PeekableTokens<'a>, name: &'a str) -> Result<'a, ExpressionNode<'a>> {
        SyntaxTree::confirm_kind(TokenKind::OpenParen, &tokens.next()?, tokens)?;
        let arguments: Vec<ExpressionNode<'a>> = if tokens.peek()?.kind == TokenKind::CloseParen {
            tokens.next()?;
            Vec::new()
        } else {
            let arguments = SyntaxTree::call_arguments(tokens)?;
            SyntaxTree::confirm_kind(TokenKind::CloseParen, &tokens.next()?, tokens)?;
            arguments
        };
        
        Ok(ExpressionNode::FunctionCall { name, arguments })
    }

    fn call_arguments(tokens: &mut PeekableTokens<'a>) -> Result<'a, Vec<ExpressionNode<'a>>> {
        SyntaxTree::skip_newlines(tokens)?;
        let mut expressions = vec![SyntaxTree::expression(tokens)?];

        while tokens.peek()?.kind == TokenKind::Comma {
            tokens.next()?;
            SyntaxTree::skip_newlines(tokens)?;
            if tokens.peek()?.kind == TokenKind::CloseParen { break; }
            expressions.push(SyntaxTree::expression(tokens)?);
        }

        Ok(expressions)
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
