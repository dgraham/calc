use std::rc::Rc;

use error::ParseError;
use node::Node;
use parser::Parser;
use scanner::{Scanner, Token};

mod error;
mod parser;
mod node;
mod scanner;

pub fn eval(text: &str) -> Result<f64, ParseError> {
    let scanner = Scanner::new(text);
    let tokens: Vec<Token> = scanner.collect();
    let parser = Parser::new();
    let expr = try!(parser.expression(&tokens));
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
                let mut nodes = node.children();
                nodes.reverse();
                self.stack.append(&mut nodes);
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{eval, Iter};
    use error::ParseError;
    use parser::Parser;
    use scanner::{Scanner, Token};

    #[test]
    fn it_adds() {
        assert_eq!(3.0, eval("1 + 2").unwrap());
    }

    #[test]
    fn it_multiplies() {
        assert_eq!(16.0, eval("2 * 8").unwrap());
    }

    #[test]
    fn it_enforces_operation_order() {
        assert_eq!(20.0, eval("4 + 2 * 8").unwrap());
    }

    #[test]
    fn it_groups_terms() {
        assert_eq!(3.0, eval("((((5)+2)*2)-5)/3").unwrap());
    }

    #[test]
    fn it_negates_values() {
        assert_eq!(-18.0, eval("6 * -3").unwrap());
    }

    #[test]
    fn it_negates_groups() {
        assert_eq!(-12.0, eval("-(5 * 2) - 2").unwrap());
    }

    #[test]
    fn it_parses_multiple_digits() {
        assert_eq!(42.0, eval("1 + 41").unwrap());
    }

    #[test]
    fn it_parses_embedded_zero() {
        assert_eq!(103.0, eval("1 + 102").unwrap());
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
        let parser = Parser::new();
        let expr = parser.expression(&tokens).unwrap();
        let iter = Iter::new(expr.node);
        let mapped: Vec<String> = iter.map(|node| node.to_string()).collect();
        assert_eq!(vec!["+", "1", "*", "-", "2", "3", "/", "4", "*", "5", "6"],
                   mapped);
    }
}
