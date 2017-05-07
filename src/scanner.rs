use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Digit(u32),
    Plus,
    Minus,
    Star,
    Solidus,
    LeftParen,
    RightParen,
    Unrecognized(char),
}

impl TokenKind {
    pub fn is_digit(&self) -> bool {
        match *self {
            TokenKind::Digit(_) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> u64 {
        match *self {
            TokenKind::Digit(value) => value as u64,
            _ => 0,
        }
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

impl Token {
    fn new(kind: TokenKind, position: usize) -> Self {
        Token {
            kind: kind,
            position: position,
        }
    }
}

pub struct Scanner<'a> {
    chars: Enumerate<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        Scanner { chars: text.chars().enumerate() }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.chars.next() {
            Some((ix, ch)) => {
                match ch {
                    '0'...'9' => Some(Token::new(TokenKind::Digit(ch.to_digit(10).unwrap()), ix)),
                    '+' => Some(Token::new(TokenKind::Plus, ix)),
                    '-' => Some(Token::new(TokenKind::Minus, ix)),
                    '*' => Some(Token::new(TokenKind::Star, ix)),
                    '/' => Some(Token::new(TokenKind::Solidus, ix)),
                    '(' => Some(Token::new(TokenKind::LeftParen, ix)),
                    ')' => Some(Token::new(TokenKind::RightParen, ix)),
                    ' ' | '\n' | '\t' => self.next(),
                    _ => Some(Token::new(TokenKind::Unrecognized(ch), ix)),
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token, TokenKind};

    #[test]
    fn it_scans() {
        let scanner = Scanner::new("1 + 2");
        let tokens: Vec<Token> = scanner.collect();
        let kinds: Vec<&TokenKind> = tokens.iter().map(|token| &token.kind).collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![&TokenKind::Digit(1), &TokenKind::Plus, &TokenKind::Digit(2)],
                   kinds);
    }

    #[test]
    fn it_scans_unrecognized_tokens() {
        let scanner = Scanner::new("1 a 2");
        let tokens: Vec<Token> = scanner.collect();
        let kinds: Vec<&TokenKind> = tokens.iter().map(|token| &token.kind).collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![&TokenKind::Digit(1),
                        &TokenKind::Unrecognized('a'),
                        &TokenKind::Digit(2)],
                   kinds);
    }

    #[test]
    fn it_ignores_whitespace() {
        let scanner = Scanner::new("\t 1 \n\n + 2 \t");
        let tokens: Vec<Token> = scanner.collect();
        let kinds: Vec<&TokenKind> = tokens.iter().map(|token| &token.kind).collect();
        assert_eq!(3, tokens.len());
        assert_eq!(vec![&TokenKind::Digit(1), &TokenKind::Plus, &TokenKind::Digit(2)],
                   kinds);
    }
}
