use std::rc::Rc;

use error::ParseError;
use scanner::{Token, TokenKind};
use node::{BinaryOp, Constant, Node, UnaryOp};

pub struct Partial<'a> {
    pub node: Rc<Node>,
    pub tokens: &'a [Token],
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Partial, ParseError> {
        expression(tokens)
    }
}

fn expression(tokens: &[Token]) -> Result<Partial, ParseError> {
    let term = try!(term(tokens));

    match term.tokens.split_first() {
        Some((token, tokens)) => {
            match token.kind {
                TokenKind::Plus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Rc::new(BinaryOp::add(token.position, term.node, expr.node)),
                        tokens: expr.tokens,
                    })
                }
                TokenKind::Minus => {
                    let expr = try!(expression(tokens));
                    Ok(Partial {
                        node: Rc::new(BinaryOp::subtract(token.position, term.node, expr.node)),
                        tokens: expr.tokens,
                    })
                }
                TokenKind::Unrecognized(_) => Err(ParseError::InvalidToken(token.position)),
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
            match token.kind {
                TokenKind::Star => {
                    let id = token.position;
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Rc::new(BinaryOp::multiply(id, factor.node, term.node)),
                        tokens: term.tokens,
                    })
                }
                TokenKind::Solidus => {
                    let term = try!(term(tokens));
                    Ok(Partial {
                        node: Rc::new(BinaryOp::divide(token.position, factor.node, term.node)),
                        tokens: term.tokens,
                    })
                }
                TokenKind::Unrecognized(_) => Err(ParseError::InvalidToken(token.position)),
                _ => Ok(factor),
            }
        }
        None => Ok(factor),
    }
}

fn integer(tokens: &[Token]) -> Option<Partial> {
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
                node: Rc::new(Constant::new(tokens[0].position, sum)),
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
            match token.kind {
                TokenKind::Minus => {
                    let factor = try!(factor(tokens));
                    Ok(Partial {
                        node: Rc::new(UnaryOp::negate(token.position, factor.node)),
                        tokens: factor.tokens,
                    })
                }
                TokenKind::LeftParen => {
                    let expr = try!(expression(tokens));
                    match expr.tokens.split_first() {
                        Some((token, tokens)) => {
                            match token.kind {
                                TokenKind::RightParen => {
                                    Ok(Partial {
                                        node: expr.node,
                                        tokens: tokens,
                                    })
                                }
                                _ => Err(ParseError::InvalidGroup(token.position)),
                            }
                        }
                        None => Err(ParseError::UnexpectedEof),
                    }
                }
                _ => Err(ParseError::FactorExpected(token.position)),
            }
        }
        None => Err(ParseError::UnexpectedEof),
    }
}
