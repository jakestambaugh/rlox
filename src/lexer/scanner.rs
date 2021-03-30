use crate::lexer::token::{LoxIdentifier, LoxNumber, LoxString, Token, TokenType};
use std::collections::HashMap;

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
    source: &'a [u8],
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: u32,
}

// Consider std::io::BufRead since it has methods like `consume` and `read_until`
impl Scanner<'_> {
    /* Wraps a string in a scanner
     */
    pub fn from_source<'a>(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.as_bytes(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::from(""), self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let token_type = match c {
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
                if self.match_current('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_current('=') {
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
                if self.match_current('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    TokenType::Skip
                } else if self.match_current('*') {
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        self.advance();
                    }
                    // Consume the closing */ characters.
                    self.advance();
                    self.advance();
                    TokenType::Skip
                } else {
                    TokenType::Slash
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                identifier.push(c);
                while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
                    identifier.push(self.advance());
                }
                TokenType::Identifier(LoxIdentifier(identifier))
            }
            '0'..='9' => {
                let mut numeral = String::new();
                // This feels inelegant, but necessary
                numeral.push(c);
                while self.peek().is_ascii_digit() {
                    numeral.push(self.advance());
                }
                // Look for fractional part
                if self.peek() == '.' && self.peek_next().is_ascii_digit() {
                    // Push the .
                    numeral.push(self.advance());
                    // Push the fraction part of the number
                    while self.peek().is_ascii_digit() {
                        numeral.push(self.advance());
                    }
                }
                TokenType::Number(LoxNumber(numeral.parse().unwrap()))
            }
            '"' => {
                let mut lexeme = String::new();
                while self.peek() != '"' {
                    lexeme.push(self.advance());
                }
                TokenType::String(LoxString(lexeme))
            }
            ' ' => TokenType::Skip,
            _ => TokenType::EOF,
        };

        let text: String =
            String::from_utf8(self.source.get(self.start..self.current).unwrap().to_vec()).unwrap();

        if token_type != TokenType::Skip {
            self.tokens.push(Token::new(token_type, text, self.line))
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        // https://users.rust-lang.org/t/accessing-the-char-at-a-byte-index/15398
        self.current += 1;
        self.source[self.current - 1] as char
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current] as char
        }
    }

    fn peek_next(&mut self) -> char {
        self.source[self.current + 1] as char
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            true
        } else if self.source[self.current] as char != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
}
