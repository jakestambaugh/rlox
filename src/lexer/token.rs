#[derive(Debug, PartialEq, Clone)]
pub struct LoxIdentifier(pub String);
#[derive(Debug, PartialEq, Clone)]
pub struct LoxString(pub String);
#[derive(Debug, PartialEq, Clone)]
pub struct LoxNumber(pub f64);

#[derive(Debug, PartialEq)]
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
    Identifier(LoxIdentifier),
    String(LoxString),
    Number(LoxNumber),

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

    // TODO: this skip should probably be removed in favor of an
    // Option<TokenType> since it represents a line of source that should be
    // skipped by the parser. Right now it gets triggered by spaces and
    // comments.
    Skip,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            token_type: token_type,
            lexeme,
            line,
        }
    }
}
