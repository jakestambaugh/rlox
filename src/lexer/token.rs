use crate::lexer::literal::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(Literal),
    String(Literal),
    Number(Literal),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: u32) -> Token {
        Token {
            token_type: token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }
}
