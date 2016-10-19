use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Digit(u32),
    Plus,
    Minus,
    Star,
    Solidus,
    LeftParen,
    RightParen,
    Unrecognized(char),
}

impl Token {
    pub fn is_digit(&self) -> bool {
        match *self {
            Token::Digit(_) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> u64 {
        match *self {
            Token::Digit(value) => value as u64,
            _ => 0,
        }
    }
}

pub struct Scanner<'a> {
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        Scanner { chars: text.chars() }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.chars.next() {
            Some(ch) => {
                match ch {
                    '0'...'9' => Some(Token::Digit(ch.to_digit(10).unwrap())),
                    '+' => Some(Token::Plus),
                    '-' => Some(Token::Minus),
                    '*' => Some(Token::Star),
                    '/' => Some(Token::Solidus),
                    '(' => Some(Token::LeftParen),
                    ')' => Some(Token::RightParen),
                    ' ' | '\n' | '\t' => self.next(),
                    _ => Some(Token::Unrecognized(ch)),
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token};

    #[test]
    fn it_scans() {
        let scanner = Scanner::new("1 + 2");
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Plus, Token::Digit(2)], tokens);
    }

    #[test]
    fn it_scans_unrecognized_tokens() {
        let scanner = Scanner::new("1 a 2");
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Unrecognized('a'), Token::Digit(2)],
                   tokens);
    }

    #[test]
    fn it_ignores_whitespace() {
        let scanner = Scanner::new("\t 1 \n\n + 2 \t");
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Plus, Token::Digit(2)], tokens);
    }
}
