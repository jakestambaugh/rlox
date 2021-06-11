use crate::lexer::literal::Literal;
use crate::lexer::token::{Token, TokenType};
use std::iter::FromIterator;
use std::iter::Peekable;
use std::{collections::HashMap, str::Chars};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords
    };
}

/** Scanner wraps a string of source code characters without taking ownership. The lifetime
   annotation 'a says that the Scanner has the same lifetime as the source string.
   It may make more sense to turn this into an Iterator or a Stream of characters in the future
   so that the entire source file doesn't have to stay in memory throughout parsing.

   The scanner should own the vector of new tokens until the end of parsing.
*/
pub struct Scanner<'a> {
    tokens: Vec<Token>,

    // Both "current" and "lookahead" are iterators over the characters in the source
    // string. The Scanner owns these iterators, but not the string that they are iterating
    // over. We have an invariant that lookahead is always one element ahead of current, so
    // we will always increment them together.
    current: Peekable<Chars<'a>>,
    line: u32,
}

impl Scanner<'_> {
    /* Wraps a string in a scanner
     */
    pub fn from_source<'a>(source: &'a str) -> Scanner<'a> {
        Scanner {
            tokens: Vec::new(),

            current: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.current.peek().is_some() {
            // We are at the beginning of the next lexeme.
            // self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::from(""), self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let token_type = match self.current.next().unwrap() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,

            '!' => {
                if self.current.peek() == Some(&'=') {
                    self.current.next();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.current.peek() == Some(&'=') {
                    self.current.next();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '>' => {
                if self.current.peek() == Some(&'=') {
                    self.current.next();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if self.current.peek() == Some(&'=') {
                    self.current.next();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '/' => {
                if self.current.peek() == Some(&'/') {
                    while self.current.peek() != Some(&'\n') && self.current.next().is_some() {
                        self.current.next();
                    }
                    TokenType::Skip
                } else if self.current.peek() == Some(&'*') {
                    let mut just_consumed: char = '\0';
                    while !(just_consumed == '*' && self.current.peek() == Some(&'/'))
                        && self.current.peek().is_some()
                    {
                        just_consumed = self.current.next().unwrap();
                    }
                    // Consume the closing /*  */ characters.
                    self.current.next();
                    TokenType::Skip
                } else {
                    TokenType::Slash
                }
            }

            'a'..='z' | 'A'..='Z' => {
                TokenType::Identifier(super::Literal::LoxIdentifier(String::from("Identifier")))
            }

            '"' => {
                let mut lexeme = vec![];
                while self.current.peek() != Some(&'"') {
                    lexeme.push(self.current.next().unwrap());
                }
                TokenType::String(Literal::LoxString(String::from_iter(lexeme)))
            }
            _ => TokenType::EOF,
        };

        // let text: String = self.current.peekable().peek();

        self.tokens
            .push(Token::new(token_type, "".to_string(), self.line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_shows_current_element() {
        let source = "1 + 2 + 3";
        let mut scanner = Scanner::from_source(source);
        assert_eq!(scanner.current.peek(), Some(&'1'))
    }
}
