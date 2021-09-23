mod error;
mod binary_operator;
mod syntax_tree;
mod generate_wasm;

use binary_operator::BinaryOperator;

pub(crate) use error::Error;
pub(crate) use syntax_tree::SyntaxTree;

#[cfg(test)]
mod tests {
    use super::super::PeekableTokens;
    use super::{SyntaxTree, BinaryOperator};

    #[test]
    fn it_works() {
        assert_eq!(
            SyntaxTree::new(PeekableTokens::new("2 + 3 * (5 - (1 + 4)) / 2")).unwrap(),
            SyntaxTree::BinaryExpr {
                lhs: Box::new(SyntaxTree::Value("2")),
                rhs: Box::new(SyntaxTree::BinaryExpr {
                    lhs: Box::new(SyntaxTree::BinaryExpr {
                        lhs: Box::new(SyntaxTree::Value("3")),
                        rhs: Box::new(SyntaxTree::BinaryExpr {
                            lhs: Box::new(SyntaxTree::Value("5")),
                            rhs: Box::new(SyntaxTree::BinaryExpr {
                                lhs: Box::new(SyntaxTree::Value("1")),
                                rhs: Box::new(SyntaxTree::Value("4")),
                                operator: BinaryOperator::Plus,
                            }),
                            operator: BinaryOperator::Minus,
                        }),
                        operator: BinaryOperator::Times,
                    }),
                    rhs: Box::new(SyntaxTree::Value("2")),
                    operator: BinaryOperator::Divide,
                }),
                operator: BinaryOperator::Plus,
            }
        )
    }


    #[test]
    fn error() {
        let expected = r#"Unexpected token found. line: 1
Expected: [OpenParen, Number]

2 + 3 * (5 - (1 + +)) / 2
                                    ^
"#;
        let err = SyntaxTree::new(PeekableTokens::new("2 + 3 * (5 - (1 + +)) / 2")).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}
