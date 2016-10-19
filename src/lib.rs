use std::error;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use scanner::{Scanner, Token};

mod scanner;

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

pub enum Node {
    Add(Rc<Node>, Rc<Node>),
    Subtract(Rc<Node>, Rc<Node>),
    Multiply(Rc<Node>, Rc<Node>),
    Divide(Rc<Node>, Rc<Node>),
    Negate(Rc<Node>),
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Add(..) => write!(f, "+"),
            Node::Subtract(..) => write!(f, "-"),
            Node::Multiply(..) => write!(f, "*"),
            Node::Divide(..) => write!(f, "/"),
            Node::Negate(..) => write!(f, "-"),
            Node::Int(value) => write!(f, "{}", value.to_string()),
        }
    }
}

pub struct Partial<'a> {
    node: Rc<Node>,
    tokens: &'a [Token],
}

pub fn expression(tokens: &[Token]) -> Result<Partial, ParseError> {
    let term = try!(term(tokens));

    match term.tokens.split_first() {
        Some((token, tokens)) => {
            match *token {
                Token::Plus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Rc::new(Node::Add(term.node, expr.node)),
                        tokens: expr.tokens,
                    })
                }
                Token::Minus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Rc::new(Node::Subtract(term.node, expr.node)),
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
                        node: Rc::new(Node::Multiply(factor.node, term.node)),
                        tokens: term.tokens,
                    })
                }
                Token::Solidus => {
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Rc::new(Node::Divide(factor.node, term.node)),
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
                node: Rc::new(Node::Int(sum)),
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
                        node: Rc::new(Node::Negate(factor.node)),
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
    let scanner = Scanner::new(text);
    let tokens: Vec<Token> = scanner.collect();
    let expr = try!(expression(&tokens));
    match expr.tokens.len() {
        0 => Ok(expr.node.value()),
        _ => Err(ParseError::UnexpectedToken),
    }
}

struct Iter {
    stack: Vec<Rc<Node>>,
}

impl Iter {
    fn new(root: Rc<Node>) -> Self {
        Iter { stack: vec![root] }
    }
}

impl Iterator for Iter {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Rc<Node>> {
        match self.stack.pop() {
            Some(node) => {
                match *node {
                    Node::Add(ref lhs, ref rhs) |
                    Node::Subtract(ref lhs, ref rhs) |
                    Node::Multiply(ref lhs, ref rhs) |
                    Node::Divide(ref lhs, ref rhs) => {
                        self.stack.push(rhs.clone());
                        self.stack.push(lhs.clone());
                    }
                    Node::Negate(ref rhs) => {
                        self.stack.push(rhs.clone());
                    }
                    Node::Int(_) => (),
                }
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{eval, expression, Iter, ParseError};
    use super::scanner::{Scanner, Token};

    #[test]
    fn it_adds() {
        assert_eq!(3 as f64, eval("1 + 2").unwrap());
    }

    #[test]
    fn it_multiplies() {
        assert_eq!(16 as f64, eval("2 * 8").unwrap());
    }

    #[test]
    fn it_enforces_operation_order() {
        assert_eq!(20 as f64, eval("4 + 2 * 8").unwrap());
    }

    #[test]
    fn it_groups_terms() {
        assert_eq!(3 as f64, eval("((((5)+2)*2)-5)/3").unwrap());
    }

    #[test]
    fn it_negates_values() {
        assert_eq!(-18 as f64, eval("6 * -3").unwrap());
    }

    #[test]
    fn it_negates_groups() {
        assert_eq!(-12 as f64, eval("-(5 * 2) - 2").unwrap());
    }

    #[test]
    fn it_parses_multiple_digits() {
        assert_eq!(42 as f64, eval("1 + 41").unwrap());
    }

    #[test]
    fn it_parses_embedded_zero() {
        assert_eq!(103 as f64, eval("1 + 102").unwrap());
    }

    #[test]
    fn it_enforces_group_close() {
        match eval("(1") {
            Err(ParseError::InvalidGroup) => (),
            _ => panic!("Must enforce closing paren"),
        }
    }

    #[test]
    fn it_enforces_missing_factor() {
        match eval("(") {
            Err(ParseError::FactorExpected) => (),
            _ => panic!("Must enforce factor grammar"),
        }
    }

    #[test]
    fn it_enforces_factor_operators() {
        match eval("1 + *") {
            Err(ParseError::FactorExpected) => (),
            _ => panic!("Must enforce factor grammar"),
        }
    }

    #[test]
    fn it_enforces_unrecognized_tokens() {
        match eval("1 a 2") {
            Err(ParseError::InvalidToken) => (),
            _ => panic!("Must enforce unrecognized tokens"),
        }
    }

    #[test]
    fn it_enforces_extra_tokens() {
        match eval("(1 + 2) 2") {
            Err(ParseError::UnexpectedToken) => (),
            _ => panic!("Must enforce extra tokens"),
        }
    }

    #[test]
    fn it_iterates() {
        let scanner = Scanner::new("1 + (2 - 3) * 4 / 5 * 6");
        let tokens: Vec<Token> = scanner.collect();
        let expr = expression(&tokens).unwrap();
        let iter = Iter::new(expr.node);
        let mapped: Vec<String> = iter.map(|node| node.to_string()).collect();
        assert_eq!(vec!["+", "1", "*", "-", "2", "3", "/", "4", "*", "5", "6"],
                   mapped);
    }
}
