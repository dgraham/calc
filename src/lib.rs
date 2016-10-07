use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    InvalidToken,
    InvalidGroup,
    FactorExpected,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedToken => "Unconsumed input",
            ParseError::InvalidToken => "Unrecognized token",
            ParseError::InvalidGroup => "Expected group close",
            ParseError::FactorExpected => "Expected integer, negation, or group",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}

enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Negate(Box<Node>),
    Int(u64),
}

impl Node {
    fn value(&self) -> f64 {
        match *self {
            Node::Add(ref lhs, ref rhs) => lhs.value() + rhs.value(),
            Node::Subtract(ref lhs, ref rhs) => lhs.value() - rhs.value(),
            Node::Multiply(ref lhs, ref rhs) => lhs.value() * rhs.value(),
            Node::Divide(ref lhs, ref rhs) => lhs.value() / rhs.value(),
            Node::Negate(ref rhs) => -rhs.value(),
            Node::Int(value) => value as f64,
        }
    }
}

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
    fn is_digit(&self) -> bool {
        match *self {
            Token::Digit(_) => true,
            _ => false,
        }
    }

    fn value(&self) -> u64 {
        match *self {
            Token::Digit(value) => value as u64,
            _ => 0,
        }
    }
}

pub struct Partial<'a> {
    node: Node,
    tokens: &'a [Token],
}

pub fn scan(text: &str) -> Vec<Token> {
    text.chars()
        .filter_map(|ch| {
            match ch {
                '0'...'9' => Some(Token::Digit(ch.to_digit(10).unwrap())),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Star),
                '/' => Some(Token::Solidus),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                ' ' | '\n' | '\t' => None,
                _ => Some(Token::Unrecognized(ch)),
            }
        })
        .collect()
}

pub fn expression(tokens: &[Token]) -> Result<Partial, ParseError> {
    let term = try!(term(tokens));

    match term.tokens.split_first() {
        Some((token, tokens)) => {
            match *token {
                Token::Plus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Node::Add(Box::new(term.node), Box::new(expr.node)),
                        tokens: expr.tokens,
                    })
                }
                Token::Minus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Node::Subtract(Box::new(term.node), Box::new(expr.node)),
                        tokens: expr.tokens,
                    })
                }
                Token::Unrecognized(_) => Err(ParseError::InvalidToken),
                _ => Ok(term),
            }
        }
        None => Ok(term),
    }
}

fn term(tokens: &[Token]) -> Result<Partial, ParseError> {
    let factor = try!(factor(tokens));

    match factor.tokens.split_first() {
        Some((token, tokens)) => {
            match *token {
                Token::Star => {
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Node::Multiply(Box::new(factor.node), Box::new(term.node)),
                        tokens: term.tokens,
                    })
                }
                Token::Solidus => {
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Node::Divide(Box::new(factor.node), Box::new(term.node)),
                        tokens: term.tokens,
                    })
                }
                Token::Unrecognized(_) => Err(ParseError::InvalidToken),
                _ => Ok(factor),
            }
        }
        None => Ok(factor),
    }
}

fn integer(tokens: &[Token]) -> Option<Partial> {
    let digits: Vec<u64> = tokens.iter()
        .take_while(|token| token.is_digit())
        .map(|token| token.value())
        .collect();

    let sum = digits.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (ix, digit)| sum + digit * 10u64.pow(ix as u32));

    match digits.len() {
        0 => None,
        _ => {
            Some(Partial {
                node: Node::Int(sum),
                tokens: &tokens[digits.len()..],
            })
        }
    }
}

fn factor(tokens: &[Token]) -> Result<Partial, ParseError> {
    if let Some(integer) = integer(tokens) {
        return Ok(integer);
    }

    match tokens.split_first() {
        Some((token, tokens)) => {
            match *token {
                Token::Minus => {
                    let factor = try!(factor(tokens));
                    Ok(Partial {
                        node: Node::Negate(Box::new(factor.node)),
                        tokens: factor.tokens,
                    })
                }
                Token::LeftParen => {
                    let expr = try!(expression(tokens));
                    match expr.tokens.split_first() {
                        Some((token, tokens)) => {
                            match *token {
                                Token::RightParen => {
                                    Ok(Partial {
                                        node: expr.node,
                                        tokens: tokens,
                                    })
                                }
                                _ => Err(ParseError::InvalidGroup),
                            }
                        }
                        None => Err(ParseError::InvalidGroup),
                    }
                }
                _ => Err(ParseError::FactorExpected),
            }
        }
        None => Err(ParseError::FactorExpected),
    }
}

pub fn eval(text: &str) -> Result<f64, ParseError> {
    let tokens = scan(text);
    let expr = try!(expression(&tokens));
    match expr.tokens.len() {
        0 => Ok(expr.node.value()),
        _ => Err(ParseError::UnexpectedToken),
    }
}

#[cfg(test)]
mod tests {
    use super::{eval, expression, scan, ParseError, Token};

    #[test]
    fn it_scans() {
        let tokens = scan("1 + 2");
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Plus, Token::Digit(2)], tokens);
    }

    #[test]
    fn it_scans_unrecognized_tokens() {
        let tokens = scan("1 a 2");
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Unrecognized('a'), Token::Digit(2)],
                   tokens);
    }

    #[test]
    fn it_ignores_whitespace() {
        let tokens = scan("\t 1 \n\n + 2 \t");
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Digit(1), Token::Plus, Token::Digit(2)], tokens);
    }

    #[test]
    fn it_adds() {
        let tokens = scan("1 + 2");
        let expr = expression(&tokens).unwrap();
        assert_eq!(3 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_multiplies() {
        let tokens = scan("2 * 8");
        let expr = expression(&tokens).unwrap();
        assert_eq!(16 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_enforces_operation_order() {
        let tokens = scan("4 + 2 * 8");
        let expr = expression(&tokens).unwrap();
        assert_eq!(20 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_groups_terms() {
        let tokens = scan("((((5)+2)*2)-5)/3");
        let expr = expression(&tokens).unwrap();
        assert_eq!(3 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_negates_values() {
        let tokens = scan("6 * -3");
        let expr = expression(&tokens).unwrap();
        assert_eq!(-18 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_negates_groups() {
        let tokens = scan("-(5 * 2) - 2");
        let expr = expression(&tokens).unwrap();
        assert_eq!(-12 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_parses_multiple_digits() {
        let tokens = scan("1 + 41");
        let expr = expression(&tokens).unwrap();
        assert_eq!(42 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_parses_embedded_zero() {
        let tokens = scan("1 + 102");
        let expr = expression(&tokens).unwrap();
        assert_eq!(103 as f64, expr.node.value());
        assert_eq!(0, expr.tokens.len());
    }

    #[test]
    fn it_enforces_group_close() {
        let tokens = scan("(1");
        match expression(&tokens) {
            Err(ParseError::InvalidGroup) => (),
            _ => panic!("Must enforce closing paren"),
        }
    }

    #[test]
    fn it_enforces_missing_factor() {
        let tokens = scan("(");
        match expression(&tokens) {
            Err(ParseError::FactorExpected) => (),
            _ => panic!("Must enforce factor grammar"),
        }
    }

    #[test]
    fn it_enforces_factor_operators() {
        let tokens = scan("1 + *");
        match expression(&tokens) {
            Err(ParseError::FactorExpected) => (),
            _ => panic!("Must enforce factor grammar"),
        }
    }

    #[test]
    fn it_enforces_unrecognized_tokens() {
        let tokens = scan("1 a 2");
        match expression(&tokens) {
            Err(ParseError::InvalidToken) => (),
            _ => panic!("Must enforce unrecognized tokens"),
        }
    }

    #[test]
    fn it_evaluates_input() {
        match eval("1 + 2") {
            Ok(value) => assert_eq!(3 as f64, value),
            Err(_) => panic!("Must eval expression"),
        }
    }

    #[test]
    fn it_enforces_extra_tokens() {
        match eval("(1 + 2) 2") {
            Err(ParseError::UnexpectedToken) => (),
            _ => panic!("Must enforce extra tokens"),
        }
    }
}
