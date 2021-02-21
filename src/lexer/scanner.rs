use crate::lexer::literal::Literal;
use crate::lexer::token::{Token, TokenType};
use std::{collections::HashMap, str::Chars};
use std::iter::FromIterator;

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
    // https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
    source: &'a str,
    tokens: Vec<Token>,

    // Both "current" and "lookahead" are iterators over the characters in the source
    // string. The Scanner owns these iterators, but not the string that they are iterating
    // over. We have an invariant that lookahead is always one element ahead of current, so
    // we will always increment them together.
    current: Chars<'a>,
    lookahead: Chars<'a>,
    line: u32,
}

impl Iterator for Scanner<'_> {
    // We are going to return the current character and the next character, since the Lox language
    // requires LL(1) lookahead. The lookahead will drain before the current iterator.
    type Item = (char, Option<char>);

    // https://docs.rs/itertools/0.10.0/itertools/trait.Itertools.html#method.tuple_windows
    // I'm kind of copying the behavior of itertools::tuple_window, but I want slightly
    // different behavior when the lookahead iterator is drained but the current isn't.
    fn next(&mut self) -> Option<Self::Item> {
        let pair = (self.current.next(), self.lookahead.next());
        match pair {
            (Some(x), Some(y)) => Some((x, Some(y))),
            (Some(x), None) => Some((x, None)), // We are at the last character in our source
            (None, Some(x)) => None, // Somehow lookahead has a character but current does not. This should be a panic
            (None, None) => None
        }
    }
}

impl Scanner<'_> {
    /* Wraps a string in a scanner
     */
    pub fn from_source<'a>(source: &'a str) -> Scanner<'a> {
        let mut lookahead = source.chars();
        lookahead.next();
        Scanner {
            source,
            tokens: Vec::new(),

            current: source.chars(),
            lookahead,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            // self.start = self.current;
            let (curr, look) = match self.next() {
                Some((x, y)) => (x, y),
                None => ('\0', None)
            };
            self.scan_token(curr, look);
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::from(""), self.line));
        &self.tokens
    }

    fn scan_token(&mut self, start: char, lookahead: Option<char>) {
        let token_type = match start {
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
                if lookahead == Some('=') {
                    self.next();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if lookahead == Some('=') {
                    self.next();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '>' => {
                if self.match_current('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if self.match_current('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '/' => {
                /*
                if self.match_current('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    TokenType::Skip
                } else if self.match_current('*') {
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        self.advance();
                    }
                    // Consume the closing /*  */ characters.
                    self.advance();
                    self.advance();
                    TokenType::Skip
                } else {
                    TokenType::Slash
                }
                */
                TokenType::Skip
            }

            'a'..='z' | 'A'..='Z' => {
                TokenType::Identifier(super::Literal::LoxIdentifier(String::from("Identifier")))
            }

            '"' => {
                let mut lexeme = vec![];
                while self.peek() != Some(&'"') {
                    lexeme.push(self.advance());
                }
                TokenType::String(Literal::LoxString(String::from_iter(lexeme)))
            }
            _ => TokenType::EOF,
        };

        // let text: String = self.current.peekable().peek();

        self.tokens.push(Token::new(token_type, "".to_string(), self.line))
    }

    fn is_at_end(&self) -> bool {
        return self.current.peekable().peek().is_none();
    }

    fn advance(&mut self) -> char {
        // https://users.rust-lang.org/t/accessing-the-char-at-a-byte-index/15398
        let c = self.current.next();
        match c {
            Some(x) => x,
            None => '\0'
        }
    }

    fn peek(&mut self) -> Option<&char> {
        let x = self.current.peekable().peek()
    }

    fn peek_next(&mut self) -> Option<&char> {
        self.lookahead.peekable().peek()
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            true
        } else if self.current.peekable().peek() != Some(&expected) {
            false
        } else {
            self.current.next();
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_shows_current_element() {
        let source = "1 + 2 + 3";
        let scanner = Scanner::from_source(source);
        assert_eq!(scanner.peek(), Some(&'1'))
    }
}
