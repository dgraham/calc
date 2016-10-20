use std::fmt;
use std::rc::Rc;

use error::ParseError;
use scanner::Token;

pub enum Node {
    Add(Rc<Node>, Rc<Node>),
    Subtract(Rc<Node>, Rc<Node>),
    Multiply(Rc<Node>, Rc<Node>),
    Divide(Rc<Node>, Rc<Node>),
    Negate(Rc<Node>),
    Int(u64),
}

impl Node {
    pub fn value(&self) -> f64 {
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
    pub node: Rc<Node>,
    pub tokens: &'a [Token],
}

pub struct Parser {
    id: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser { id: 0 }
    }

    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    pub fn expression<'a>(&'a self, tokens: &'a [Token]) -> Result<Partial, ParseError> {
        let term = try!(self.term(tokens));

        match term.tokens.split_first() {
            Some((token, tokens)) => {
                match *token {
                    Token::Plus => {
                        let expr = try!(self.expression(tokens));
                        Ok(Partial {
                            node: Rc::new(Node::Add(term.node, expr.node)),
                            tokens: expr.tokens,
                        })
                    }
                    Token::Minus => {
                        let expr = try!(self.expression(tokens));
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

    fn term<'a>(&'a self, tokens: &'a [Token]) -> Result<Partial, ParseError> {
        let factor = try!(self.factor(tokens));

        match factor.tokens.split_first() {
            Some((token, tokens)) => {
                match *token {
                    Token::Star => {
                        let term = try!(self.term(tokens));
                        Ok(Partial {
                            node: Rc::new(Node::Multiply(factor.node, term.node)),
                            tokens: term.tokens,
                        })
                    }
                    Token::Solidus => {
                        let term = try!(self.term(tokens));
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

    fn integer<'a>(&'a self, tokens: &'a [Token]) -> Option<Partial> {
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

    fn factor<'a>(&'a self, tokens: &'a [Token]) -> Result<Partial, ParseError> {
        if let Some(integer) = self.integer(tokens) {
            return Ok(integer);
        }

        match tokens.split_first() {
            Some((token, tokens)) => {
                match *token {
                    Token::Minus => {
                        let factor = try!(self.factor(tokens));
                        Ok(Partial {
                            node: Rc::new(Node::Negate(factor.node)),
                            tokens: factor.tokens,
                        })
                    }
                    Token::LeftParen => {
                        let expr = try!(self.expression(tokens));
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
}
