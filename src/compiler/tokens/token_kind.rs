#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Identifier,
    Number,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Times,
    Divide,
    Assign,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    NewLine,
    EOF,
}

// impl std::fmt::Display for TokenKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TokenKind::Identifier => todo!(),
//             TokenKind::Number => todo!(),
//             TokenKind::OpenParen => todo!(),
//             TokenKind::CloseParen => todo!(),
//             TokenKind::Plus => todo!(),
//             TokenKind::Minus => todo!(),
//             TokenKind::Times => todo!(),
//             TokenKind::Divide => todo!(),
//             TokenKind::Assign => todo!(),
//             TokenKind::Equal => todo!(),
//             TokenKind::Greater => todo!(),
//             TokenKind::GreaterOrEqual => todo!(),
//             TokenKind::Less => todo!(),
//             TokenKind::LessOrEqual => todo!(),
//             TokenKind::NewLine => todo!(),
//             TokenKind::EOF => todo!(),
//         }
//     }
// }
