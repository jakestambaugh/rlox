use crate::lexer::Token;
/*
EXPR:
    "Assign   : Token name, Expr value",
    "Binary   : Expr left, Token operator, Expr right",
    "Call     : Expr callee, Token paren, List<Expr> arguments",
    "Get      : Expr object, Token name",
    "Grouping : Expr expression",
    "Literal  : Object value",
    "Logical  : Expr left, Token operator, Expr right",
    "Set      : Expr object, Token name, Expr value",
    "Super    : Token keyword, Token method",
    "This     : Token keyword",
    "Unary    : Token operator, Expr right",
    "Variable : Token name"
*/

pub enum Expr {
    Assign {
        name: Token,
        value: Expr,
    },
    Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Call {
        callee: Expr,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Expr,
        name: Token,
    },
    Grouping {
        expression: Expr,
    },
    NumberLiteral {
        value: LoxNumber,
    },
    IdentifierLiteral {
        value: LoxIdentifier,
    },
    StringLiteral {
        value: LoxString,
    },
    Logical {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Set {
        object: Expr,
        name: Token,
        value: Expr,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Expr,
    },
    Variable {
        name: Token,
    },
}