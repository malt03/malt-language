#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Identifier,
    Number,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    NewLine,
    Function,
    Return,
    EOF,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Identifier => "identifier".into(),
            TokenKind::Number => "number".into(),
            TokenKind::OpenParen => "'('".into(),
            TokenKind::CloseParen => "')'".into(),
            TokenKind::OpenBrace => "'{'".into(),
            TokenKind::CloseBrace => "'}'".into(),
            TokenKind::Plus => "'+'".into(),
            TokenKind::Minus => "'-'".into(),
            TokenKind::Multiply => "'*'".into(),
            TokenKind::Divide => "'/'".into(),
            TokenKind::Assign => "'='".into(),
            TokenKind::Equal => "'=='".into(),
            TokenKind::Greater => "'>'".into(),
            TokenKind::GreaterOrEqual => "'>='".into(),
            TokenKind::Less => "'<'".into(),
            TokenKind::LessOrEqual => "'<='".into(),
            TokenKind::NewLine => "newline".into(),
            TokenKind::Function => "'fn'".into(),
            TokenKind::Return => "'return'".into(),
            TokenKind::EOF => "eof".into(),
        }
    }
}
