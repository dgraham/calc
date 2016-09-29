use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError<'a> {
    Grammar(&'a str),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Grammar(reason) => reason.fmt(f),
        }
    }
}

impl<'a> error::Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::Grammar(reason) => reason,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::Grammar(_) => None,
        }
    }
}

pub trait Node {
    fn value(&self) -> f64;
}

struct AddNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

impl Node for AddNode {
    fn value(&self) -> f64 {
        self.lhs.value() + self.rhs.value()
    }
}

struct SubtractNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

impl Node for SubtractNode {
    fn value(&self) -> f64 {
        self.lhs.value() - self.rhs.value()
    }
}

struct MultiplyNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

impl Node for MultiplyNode {
    fn value(&self) -> f64 {
        self.lhs.value() * self.rhs.value()
    }
}

struct DivideNode {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

impl Node for DivideNode {
    fn value(&self) -> f64 {
        self.lhs.value() / self.rhs.value()
    }
}

struct NegationNode {
    rhs: Box<Node>,
}

impl Node for NegationNode {
    fn value(&self) -> f64 {
        -self.rhs.value()
    }
}

struct IntNode {
    value: u32,
}

impl Node for IntNode {
    fn value(&self) -> f64 {
        self.value as f64
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Star,
    Solidus,
    LeftParen,
    RightParen,
    Whitespace,
    Unrecognized(char),
}

pub struct Partial<'a> {
    node: Box<Node>,
    tokens: &'a [Token],
}

pub fn scan(text: &str) -> Vec<Token> {
    text.chars()
        .filter_map(|ch| {
            match ch {
                '0'...'9' => Some(Token::Number(ch.to_digit(10).unwrap())),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Star),
                '/' => Some(Token::Solidus),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                ' ' => None,
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
                        node: Box::new(AddNode {
                            lhs: term.node,
                            rhs: expr.node,
                        }),
                        tokens: expr.tokens,
                    })
                }
                Token::Minus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Box::new(SubtractNode {
                            lhs: term.node,
                            rhs: expr.node,
                        }),
                        tokens: expr.tokens,
                    })
                }
                Token::Unrecognized(_) => Err(ParseError::Grammar("Unrecognized token")),
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
                        node: Box::new(MultiplyNode {
                            lhs: factor.node,
                            rhs: term.node,
                        }),
                        tokens: term.tokens,
                    })
                }
                Token::Solidus => {
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Box::new(DivideNode {
                            lhs: factor.node,
                            rhs: term.node,
                        }),
                        tokens: term.tokens,
                    })
                }
                Token::Unrecognized(_) => Err(ParseError::Grammar("Unrecognized token")),
                _ => Ok(factor),
            }
        }
        None => Ok(factor),
    }
}

fn factor(tokens: &[Token]) -> Result<Partial, ParseError> {
    match tokens.split_first() {
        Some((token, tokens)) => {
            match *token {
                Token::Number(value) => {
                    Ok(Partial {
                        node: Box::new(IntNode { value: value }),
                        tokens: tokens,
                    })
                }
                Token::Minus => {
                    let factor = try!(factor(tokens));
                    Ok(Partial {
                        node: Box::new(NegationNode { rhs: factor.node }),
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
                                _ => Err(ParseError::Grammar("Expected group close")),
                            }
                        }
                        None => Err(ParseError::Grammar("Expected group close")),
                    }
                }
                _ => Err(ParseError::Grammar("Expected integer, negation, or group")),
            }
        }
        None => Err(ParseError::Grammar("Expected factor")),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::{expression, scan, Token};

    #[test]
    fn it_scans() {
        let tokens = scan("1 + 2");
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Number(1), Token::Plus, Token::Number(2)],
                   tokens);
    }

    #[test]
    fn it_scans_unrecognized_tokens() {
        let tokens = scan("1 a 2");
        assert_eq!(3, tokens.len());
        assert_eq!(vec![Token::Number(1), Token::Unrecognized('a'), Token::Number(2)],
                   tokens);
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
    fn it_enforces_group_close() {
        let tokens = scan("(1");
        match expression(&tokens) {
            Ok(_) => panic!("Must enforce closing paren"),
            Err(e) => assert_eq!("Expected group close", e.description()),
        }
    }

    #[test]
    fn it_enforces_missing_factor() {
        let tokens = scan("(");
        match expression(&tokens) {
            Ok(_) => panic!("Must enforce factor grammar"),
            Err(e) => assert_eq!("Expected factor", e.description()),
        }
    }

    #[test]
    fn it_enforces_factor_operators() {
        let tokens = scan("1 + *");
        match expression(&tokens) {
            Ok(_) => panic!("Must enforce factor grammar"),
            Err(e) => assert_eq!("Expected integer, negation, or group", e.description()),
        }
    }

    #[test]
    fn it_enforces_unrecognized_tokens() {
        let tokens = scan("1 a 2");
        match expression(&tokens) {
            Ok(_) => panic!("Must enforce unrecognized tokens"),
            Err(e) => assert_eq!("Unrecognized token", e.description()),
        }
    }
}
