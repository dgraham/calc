use std::rc::Rc;

use node::Node;

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
    use super::Iter;
    use parser::Parser;
    use scanner::{Scanner, Token};

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
