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
                    return Some(Token::new(TokenType::String, s.clone().as_str(), self.line));
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
                    if let Some(&token_type) = KEYWORDS.get(ident.clone().as_str()) {
                        return Some(Token::new(token_type, &ident, self.line));
                    }
                    return Some(Token::new(TokenType::Identifier, ident.as_str(), self.line));
                }
                '0'..='9' => {
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
                    let _value = ident.parse::<f64>().expect("Could not parse into float");
                    return Some(Token::new(TokenType::Number, &ident, self.line));
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
