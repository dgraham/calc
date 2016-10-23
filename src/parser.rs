use std::rc::Rc;

use error::ParseError;
use scanner::{Token, TokenKind};
use node::{BinaryOp, Constant, Node, UnaryOp};

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
                match token.kind {
                    TokenKind::Plus => {
                        let expr = try!(self.expression(tokens));
                        Ok(Partial {
                            node: Rc::new(BinaryOp::add(term.node, expr.node)),
                            tokens: expr.tokens,
                        })
                    }
                    TokenKind::Minus => {
                        let expr = try!(self.expression(tokens));
                        Ok(Partial {
                            node: Rc::new(BinaryOp::subtract(term.node, expr.node)),
                            tokens: expr.tokens,
                        })
                    }
                    TokenKind::Unrecognized(_) => Err(ParseError::InvalidToken),
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
                match token.kind {
                    TokenKind::Star => {
                        let term = try!(self.term(tokens));
                        Ok(Partial {
                            node: Rc::new(BinaryOp::multiply(factor.node, term.node)),
                            tokens: term.tokens,
                        })
                    }
                    TokenKind::Solidus => {
                        let term = try!(self.term(tokens));
                        Ok(Partial {
                            node: Rc::new(BinaryOp::divide(factor.node, term.node)),
                            tokens: term.tokens,
                        })
                    }
                    TokenKind::Unrecognized(_) => Err(ParseError::InvalidToken),
                    _ => Ok(factor),
                }
            }
            None => Ok(factor),
        }
    }

    fn integer<'a>(&'a self, tokens: &'a [Token]) -> Option<Partial> {
        let digits: Vec<u64> = tokens.iter()
            .take_while(|token| token.kind.is_digit())
            .map(|token| token.kind.value())
            .collect();

        let sum = digits.iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (ix, digit)| sum + digit * 10u64.pow(ix as u32));

        match digits.len() {
            0 => None,
            _ => {
                Some(Partial {
                    node: Rc::new(Constant::new(sum)),
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
                match token.kind {
                    TokenKind::Minus => {
                        let factor = try!(self.factor(tokens));
                        Ok(Partial {
                            node: Rc::new(UnaryOp::negate(factor.node)),
                            tokens: factor.tokens,
                        })
                    }
                    TokenKind::LeftParen => {
                        let expr = try!(self.expression(tokens));
                        match expr.tokens.split_first() {
                            Some((token, tokens)) => {
                                match token.kind {
                                    TokenKind::RightParen => {
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
