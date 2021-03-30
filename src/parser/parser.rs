use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        self {
            tokens,
            current: 0,
        }
    }
}

fn expression() -> Expr {
    equality()
}

fn equality() -> Expr {
    let mut expr: Expr = comparison();

}

fn comparison() -> Expr {
}