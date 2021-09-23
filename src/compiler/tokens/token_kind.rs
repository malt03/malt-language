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

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Identifier => "identifier".into(),
            TokenKind::Number => "number".into(),
            TokenKind::OpenParen => "'('".into(),
            TokenKind::CloseParen => "')'".into(),
            TokenKind::Plus => "'+'".into(),
            TokenKind::Minus => "'-'".into(),
            TokenKind::Times => "'*'".into(),
            TokenKind::Divide => "'/'".into(),
            TokenKind::Assign => "'='".into(),
            TokenKind::Equal => "'='=".into(),
            TokenKind::Greater => "'>'".into(),
            TokenKind::GreaterOrEqual => "'>'=".into(),
            TokenKind::Less => "'<'".into(),
            TokenKind::LessOrEqual => "'<'=".into(),
            TokenKind::NewLine => "newline".into(),
            TokenKind::EOF => "eof".into(),
        }
    }
}
