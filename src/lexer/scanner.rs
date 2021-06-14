use crate::lexer::literal::Literal;
use crate::lexer::token::{Token, TokenType};
use std::iter::FromIterator;
use std::iter::Peekable;
use std::str::Chars;

/** Scanner wraps a string of source code characters without taking ownership. The lifetime
   annotation 'a says that the Scanner has the same lifetime as the source string.
   It may make more sense to turn this into an Iterator or a Stream of characters in the future
   so that the entire source file doesn't have to stay in memory throughout parsing.

   The scanner should own the vector of new tokens until the end of parsing.
*/
pub struct Scanner<'a> {
    // Both "current" and "lookahead" are iterators over the characters in the source
    // string. The Scanner owns these iterators, but not the string that they are iterating
    // over. We have an invariant that lookahead is always one element ahead of current, so
    // we will always increment them together.
    stream: Peekable<Chars<'a>>,
    line: u32,
}

impl Scanner<'_> {
    /* Wraps a string in a scanner
     */
    pub fn from_source<'a>(source: &'a str) -> Scanner<'a> {
        Scanner {
            stream: source.chars().peekable(),
            line: 1,
        }
    }

    fn tokenize_number(&mut self, ch: char) -> Option<Token> {
        let mut ident = String::from(ch);
        while let Some(&x) = self.stream.peek() {
            match x {
                '0'..='9' | '.' => {
                    ident.push(x);
                    self.stream.next();
                }
                _ => {
                    break;
                }
            }
        }
        let value = ident.parse::<f64>().expect("Could not parse into float");
        Some(Token::new(
            TokenType::Number(Literal::LoxNumber(value)),
            &ident,
            self.line,
        ))
    }
}

/// This method allows the Scanner to iterate over Tokens
impl Iterator for Scanner<'_> {
    type Item = Token;

    /// Moves forward through the stream of characters, constructing
    /// a token.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.stream.next() {
            match ch {
                '(' => return Some(Token::new(TokenType::LeftParen, "(", self.line)),
                ')' => return Some(Token::new(TokenType::RightParen, ")", self.line)),
                '{' => return Some(Token::new(TokenType::LeftBrace, "{", self.line)),
                '}' => return Some(Token::new(TokenType::RightBrace, "}", self.line)),
                ',' => return Some(Token::new(TokenType::Comma, ",", self.line)),
                '.' => return Some(Token::new(TokenType::Dot, ".", self.line)),
                '-' => return Some(Token::new(TokenType::Minus, "-", self.line)),
                '+' => return Some(Token::new(TokenType::Plus, "+", self.line)),
                ';' => return Some(Token::new(TokenType::Semicolon, ";", self.line)),
                '*' => return Some(Token::new(TokenType::Star, "*", self.line)),
                '!' => {
                    if self.stream.peek() == Some(&'=') {
                        self.stream.next();
                        return Some(Token::new(TokenType::BangEqual, "!=", self.line));
                    } else {
                        return Some(Token::new(TokenType::Bang, "=", self.line));
                    }
                }
                '=' => {
                    if self.stream.peek() == Some(&'=') {
                        self.stream.next();
                        return Some(Token::new(TokenType::EqualEqual, "==", self.line));
                    } else {
                        return Some(Token::new(TokenType::Equal, "=", self.line));
                    }
                }
                '>' => {
                    if self.stream.peek() == Some(&'=') {
                        self.stream.next();
                        return Some(Token::new(TokenType::GreaterEqual, ">=", self.line));
                    } else {
                        return Some(Token::new(TokenType::Greater, ">", self.line));
                    }
                }
                '<' => {
                    if self.stream.peek() == Some(&'=') {
                        self.stream.next();
                        return Some(Token::new(TokenType::LessEqual, "<=", self.line));
                    } else {
                        return Some(Token::new(TokenType::Less, "<", self.line));
                    }
                }
                '/' => {
                    if self.stream.peek() == Some(&'/') {
                        while self.stream.peek() != Some(&'\n') && self.stream.next().is_some() {
                            self.stream.next();
                        }
                    } else if self.stream.peek() == Some(&'*') {
                        let mut just_consumed = '\0';
                        while !(just_consumed == '*' && self.stream.peek() == Some(&'/'))
                            && self.stream.peek().is_some()
                        {
                            just_consumed = self.stream.next().unwrap();
                            if just_consumed == '\n' {
                                self.line += 1;
                            }
                        }
                        // Consume the closing /*  */ characters.
                        self.stream.next();
                    } else {
                        return Some(Token::new(TokenType::Slash, "/", self.line));
                    }
                }
                '"' => {
                    let mut lexeme = vec![];
                    while self.stream.peek() != Some(&'"') {
                        let next_ch = self.stream.next().expect("No closing quote found");
                        println!("{}", next_ch);
                        lexeme.push(next_ch);
                    }
                    // Consume closing "
                    self.stream.next();
                    let s = String::from_iter(lexeme);
                    return Some(Token::new(
                        TokenType::String(Literal::LoxIdentifier(s.clone())),
                        s.clone().as_str(),
                        self.line,
                    ));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::from(ch);
                    while let Some(&x) = self.stream.peek() {
                        match x {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
                                ident.push(x);
                                self.stream.next();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    let token = match ident.as_str() {
                        "and" => Token::new(TokenType::And, &ident, self.line),
                        "class" => Token::new(TokenType::Class, &ident, self.line),
                        "else" => Token::new(TokenType::Else, &ident, self.line),
                        "false" => Token::new(TokenType::False, &ident, self.line),
                        "for" => Token::new(TokenType::For, &ident, self.line),
                        "fun" => Token::new(TokenType::Fun, &ident, self.line),
                        "if" => Token::new(TokenType::If, &ident, self.line),
                        "nil" => Token::new(TokenType::Nil, &ident, self.line),
                        "or" => Token::new(TokenType::Or, &ident, self.line),
                        "print" => Token::new(TokenType::Print, &ident, self.line),
                        "return" => Token::new(TokenType::Return, &ident, self.line),
                        "super" => Token::new(TokenType::Super, &ident, self.line),
                        "this" => Token::new(TokenType::This, &ident, self.line),
                        "true" => Token::new(TokenType::True, &ident, self.line),
                        "var" => Token::new(TokenType::Var, &ident, self.line),
                        "while" => Token::new(TokenType::While, &ident, self.line),
                        _ => Token::new(
                            TokenType::Identifier(Literal::LoxIdentifier(ident.clone())),
                            &ident,
                            self.line,
                        ),
                    };
                    return Some(token);
                }
                '0'..='9' => {
                    return self.tokenize_number(ch);
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_shows_current_element() {
        let source = "1 + 2 + 3";
        let mut scanner = Scanner::from_source(source).peekable();
        if let Some(Token { token_type: t, .. }) = scanner.next() {
            assert_eq!(t, TokenType::Number);
        } else {
            unreachable!("This should fail");
        }
        if let Some(Token { token_type: t, .. }) = scanner.next() {
            assert_eq!(t, TokenType::Plus);
        } else {
            unreachable!("The 2nd element was not Plus")
        }
    }
}
